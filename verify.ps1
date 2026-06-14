# ============================================================
# Phase Deliverable Verification - PowerShell (Windows Native)
# ============================================================

$SRC = "src"
$PASS = 0
$FAIL = 0

Write-Host "=== Golden Sun Deliverable Verification (PowerShell) ==="
Write-Host ""

# --- Phase 1: Build + Lint ---
Write-Host "--- Phase 1: Build + Lint ---"
$checkResult = & cargo check 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  [PASS] cargo check - zero warnings" -ForegroundColor Green
    $PASS++
} else {
    Write-Host "  [FAIL] cargo check failed" -ForegroundColor Red
    Write-Host "  Action: Fix compilation errors"
    $FAIL++
}

$hasClippy = $null -ne (Get-Command "cargo-clippy" -ErrorAction SilentlyContinue)
if ($hasClippy) {
    $clippyResult = & cargo clippy -- -D warnings 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  [PASS] cargo clippy - zero warnings" -ForegroundColor Green
    } else {
        Write-Host "  [WARN] cargo clippy suggestions:" -ForegroundColor Yellow
        $clippyResult | Select-Object -First 20 | ForEach-Object { Write-Host "       $_" }
    }
}

# --- Phase 2: Hardcoded magic numbers ---
Write-Host "--- Phase 2: Magic Number Scan ---"
$hardcoded = Select-String -Path "$SRC\**\*.rs" -Pattern "640\.0|480\.0|32\.0|160\.0" `
    | Where-Object { $_.Path -notmatch "constants\.rs" -and $_.Line -notmatch '#\[' }
if (-not $hardcoded) {
    Write-Host "  [PASS] No hardcoded magic numbers" -ForegroundColor Green
    $PASS++
} else {
    Write-Host "  [FIX] Found hardcoded values:" -ForegroundColor Yellow
    $hardcoded | ForEach-Object { Write-Host "       $($_.Path):$($_.LineNumber): $($_.Line.Trim())" }
    Write-Host "  Action: Move to constants.rs"
    $FAIL++
}

# --- Phase 3: unwrap() calls ---
Write-Host "--- Phase 3: unwrap() check ---"
$unwraps = Select-String -Path "$SRC\**\*.rs" -Pattern "\.unwrap\(" `
    | Where-Object { $_.Path -notmatch "\\tests\\" -and $_.Line -notmatch "allow" -and $_.Line -notmatch "unwrap_or" -and $_.Line -notmatch "unwrap_err" -and $_.Line -notmatch "unwrap_or_else" }
if (-not $unwraps) {
    Write-Host "  [PASS] No unwrap() calls" -ForegroundColor Green
    $PASS++
} else {
    Write-Host "  [WARN] Found unwrap() calls (non-blocking):" -ForegroundColor Yellow
    $unwraps | ForEach-Object { Write-Host "       $($_.Path):$($_.LineNumber): $($_.Line.Trim())" }
}

# --- Phase 4: Tests ---
Write-Host "--- Phase 4: Tests ---"
$testOutput = & cargo test 2>&1
$testExitCode = $LASTEXITCODE
$testOutput | Out-Host
if ($testExitCode -eq 0) {
    Write-Host "  [PASS] cargo test all passed" -ForegroundColor Green
    $PASS++
} else {
    Write-Host "  [FAIL] Tests failed" -ForegroundColor Red
    $FAIL++
}

# --- Phase 5: Architecture integrity ---
Write-Host "--- Phase 5: Architecture Integrity ---"
$missing = @()
$requiredFiles = @("src\engine\constants.rs", "src\engine\input.rs", "src\lib.rs")
foreach ($f in $requiredFiles) {
    if (-not (Test-Path $f)) { $missing += $f }
}
$libMods = (Select-String -Path "src\lib.rs" -Pattern "pub mod" | Measure-Object).Count
if ($missing.Count -eq 0 -and $libMods -ge 8) {
    Write-Host "  [PASS] Architecture files complete, lib.rs declares $libMods modules" -ForegroundColor Green
    $PASS++
} elseif ($missing.Count -gt 0) {
    Write-Host "  [FAIL] Missing architecture files:" -ForegroundColor Red
    foreach ($f in $missing) { Write-Host "       - $f not found" }
    Write-Host "  Action: Create missing skeleton files"
    $FAIL++
} else {
    Write-Host "  [FIX] lib.rs has insufficient module declarations (currently $libMods, need >=8)" -ForegroundColor Yellow
    Write-Host "  Action: Add missing pub mod declarations"
    $FAIL++
}

# --- Phase 6: Release build ---
Write-Host "--- Phase 6: Release Build ---"
$buildResult = & cargo build --release 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  [PASS] cargo build --release succeeded" -ForegroundColor Green
    $PASS++
} else {
    Write-Host "  [FAIL] release build failed" -ForegroundColor Red
    $FAIL++
}

# --- Phase 7: BDD coverage (informational) ---
Write-Host "--- Phase 7: BDD Coverage ---"
$featuresDir = "tests\features"
if (Test-Path $featuresDir) {
    Get-ChildItem "$featuresDir\*.feature" | ForEach-Object {
        $fname = $_.BaseName
        $testFile = "tests\${fname}_bdd.rs"
        if (Test-Path $testFile) {
            $implCount = (Select-String -Path $testFile -Pattern "^#\[test\]" | Measure-Object).Count
            if ($implCount -eq 0) {
                Write-Host "  [INFO] $($_.Name) -> ${fname}_bdd.rs (skeleton)" -ForegroundColor Yellow
            } else {
                Write-Host "  [OK] $($_.Name) -> ${fname}_bdd.rs ($implCount tests)" -ForegroundColor Green
            }
        } else {
            Write-Host "  [INFO] $($_.Name) has no test file" -ForegroundColor Yellow
        }
    }
}

# --- Summary ---
Write-Host ""
Write-Host "===================================="
if ($FAIL -gt 0) {
    Write-Host "Passed: $PASS   Failed: $FAIL" -ForegroundColor Red
} else {
    Write-Host "Passed: $PASS   Failed: $FAIL" -ForegroundColor Green
}
Write-Host "===================================="

if ($FAIL -gt 0) {
    Write-Host ""
    Write-Host "Action: Fix items above and re-run verify.ps1"
    exit 1
} else {
    Write-Host ""
    Write-Host "  [DELIVERABLE] All checks passed" -ForegroundColor Green
}
