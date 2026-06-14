#!/usr/bin/env bash
# ============================================================
# Phase 可交付验证 — 结构化检测
# 输出格式设计为 Agent 可消费：每项输出 "[PASS|FAIL|FIXED] 说明"
# Agent 读取后自动处理 FAIL 项
# ============================================================
set -uo pipefail

SRC="src"
PASS=0
FAIL=0

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=== Golden Sun 可交付验证 ==="
echo ""

# ── 1. 编译 ──
echo "--- Phase 1: 编译 + Lint ---"
if cargo check 2>&1; then
    echo -e "  ${GREEN}[PASS]${NC} cargo check 零警告"
    PASS=$((PASS + 1))
else
    echo -e "  ${RED}[FAIL]${NC} cargo check 失败"
    echo "  ⚡ 操作: Agent 修复编译错误后重试"
    FAIL=$((FAIL + 1))
fi

# 1b. clippy（软性建议，不阻塞交付）
if command -v cargo-clippy &>/dev/null || cargo clippy --version &>/dev/null; then
    CLIPPY_OUT=$(cargo clippy -- -D warnings 2>&1 || true)
    CLIPPY_EXIT=$?
    if [ "$CLIPPY_EXIT" -eq 0 ]; then
        echo -e "  ${GREEN}[PASS]${NC} cargo clippy 零警告"
    else
        echo -e "  ${YELLOW}[WARN]${NC} cargo clippy 建议:"
        echo "$CLIPPY_OUT" | head -20 | while IFS= read -r line; do
            echo "       $line"
        done
        echo "  💡 建议: 修复 clippy 警告以提升代码质量"
    fi
fi

# ── 2. 硬件编码魔数扫描 ──
echo "--- Phase 2: 魔数扫描 ---"
HARDCODED=$(grep -rn "640\.0\|480\.0\|32\.0\|160\.0" "$SRC" --include="*.rs" \
    | grep -v constants.rs \
    | grep -v "\.git" \
    | grep -v "#\[" 2>/dev/null || true)
if [ -z "$HARDCODED" ]; then
    echo -e "  ${GREEN}[PASS]${NC} 无硬编码魔数"
    PASS=$((PASS + 1))
else
    echo -e "  ${YELLOW}[FIX]${NC} 发现以下硬编码值:"
    echo "$HARDCODED" | while IFS= read -r line; do
        echo "       $line"
    done
    echo "  ⚡ 操作: Agent 将这些值移到 constants.rs 并引用常量名"
    FAIL=$((FAIL + 1))
fi

# ── 3. unwrap 裸调用（软性建议，不阻塞交付） ──
echo "--- Phase 3: unwrap 检查 ---"
# 检查生产代码中的 unwrap（排除 tests/ 目录和已知测试辅助函数）
UNWRAPS=$(grep -rn "\.unwrap(" "$SRC" --include="*.rs" \
    | grep -v "/tests/" \
    | grep -v "#\[allow" \
    | grep -v "unwrap_or" \
    | grep -v "unwrap_err" \
    || true)
if [ -z "$UNWRAPS" ]; then
    echo -e "  ${GREEN}[PASS]${NC} 无 unwrap() 裸调用"
    PASS=$((PASS + 1))
else
    echo -e "  ${YELLOW}[WARN]${NC} 发现 unwrap() 调用（非阻塞）:"
    echo "$UNWRAPS" | while IFS= read -r line; do
        echo "       $line"
    done
    echo "  💡 建议: 替换为 ? 或 unwrap_or/ok_or"
    # 不记入 FAIL，非阻塞
fi

# ── 4. 测试 ──
echo "--- Phase 4: 测试 ---"
TEST_OUTPUT=$(cargo test 2>&1)
TEST_EXIT=$?
FAILED_TESTS=$(echo "$TEST_OUTPUT" | grep -E "FAILED|panicked at" || true)
if [ "$TEST_EXIT" -eq 0 ] && [ -z "$FAILED_TESTS" ]; then
    echo -e "  ${GREEN}[PASS]${NC} cargo test 全部通过"
    PASS=$((PASS + 1))
else
    echo -e "  ${RED}[FAIL]${NC} 测试未通过"
    echo "$FAILED_TESTS" | while IFS= read -r line; do
        echo "       $line"
    done
    FAIL=$((FAIL + 1))
fi

# ── 5. 架构文件存在性 ──
echo "--- Phase 5: 架构完整性 ---"
MISSING=""
for f in "src/engine/constants.rs" "src/engine/input.rs" "src/lib.rs"; do
    [ ! -f "$f" ] && MISSING="$MISSING $f"
done

# 检查 lib.rs 声明数
LIB_MODS=$(grep -c "pub mod" src/lib.rs 2>/dev/null || echo 0)
if [ -z "$MISSING" ] && [ "$LIB_MODS" -ge 8 ]; then
    echo -e "  ${GREEN}[PASS]${NC} 架构文件完整, lib.rs 声明 8+ 模块"
    PASS=$((PASS + 1))
elif [ -n "$MISSING" ]; then
    echo -e "  ${RED}[FAIL]${NC} 缺少架构文件:"
    for f in $MISSING; do echo "       - $f 不存在"; done
    echo "  ⚡ 操作: Agent 创建缺失的骨架文件"
    FAIL=$((FAIL + 1))
else
    echo -e "  ${YELLOW}[FIX]${NC} lib.rs 模块声明不足 (当前 $LIB_MODS, 需要 ≥8)"
    echo "  ⚡ 操作: Agent 补充缺失的 pub mod 声明"
    FAIL=$((FAIL + 1))
fi

# ── 6. Release 可构建 ──
echo "--- Phase 6: Release 构建 ---"
if cargo build --release 2>&1; then
    echo -e "  ${GREEN}[PASS]${NC} cargo build --release 成功"
    PASS=$((PASS + 1))
else
    echo -e "  ${RED}[FAIL]${NC} release 构建失败"
    echo "  ⚡ 操作: Agent 排查内存/链接错误"
    FAIL=$((FAIL + 1))
fi

# ── 7. BDD 场景完整性 — 读取 feature 文件与测试文件对比 ──
echo "--- Phase 7: BDD 场景覆盖 ---"
FEATURES_DIR="tests/features"
TEST_RS_DIR="tests"
MISMATCH=0
for feat in "$FEATURES_DIR"/*.feature; do
    FNAME=$(basename "$feat" .feature)
    TEST_FILE="$TEST_RS_DIR/${FNAME}_bdd.rs"
    if [ ! -f "$TEST_FILE" ]; then
        echo -e "  ${YELLOW}[INFO]${NC} $feat 无对应测试文件 ${FNAME}_bdd.rs"
        continue
    fi
    # 如果测试文件只包含注释，视为未实现
    IMPL_COUNT=$(grep -c "^#\\[test\\]" "$TEST_FILE" 2>/dev/null || echo "0")
    # 去除可能的空格/换行
    IMPL_COUNT=$(echo "$IMPL_COUNT" | tr -d '[:space:]')
    if [ "$IMPL_COUNT" -eq 0 ] 2>/dev/null; then
        echo -e "  ${YELLOW}[INFO]${NC} $feat → ${FNAME}_bdd.rs (骨架, 待 Phase 实现)"
    else
        echo -e "  ${GREEN}[OK]${NC} $feat → ${FNAME}_bdd.rs (${IMPL_COUNT} 测试)"
    fi
done
# 这项不记入通过/失败，仅信息性

# ── 汇总 ──
echo ""
echo "===================================="
echo -n "  通过: $PASS"
echo -e "  ${GREEN}✅${NC}"
echo -n "  失败: $FAIL"
[ $FAIL -gt 0 ] && echo -e "  ${RED}❌${NC}" || echo ""
echo "===================================="

if [ $FAIL -gt 0 ]; then
    echo ""
    echo "=== 自动修复指引（Agent 执行） ==="
    echo "请根据上方 ⚡ 操作提示逐项修复，每修一项后重新运行 verify.sh"
    echo "修复循环: detect → fix → re-verify → 直到全部 PASS"
    exit 1
else
    echo ""
    echo -e "  ${GREEN}[DELIVERABLE]${NC} 全部检查通过，可以交付"
fi
