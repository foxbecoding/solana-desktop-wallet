# File: watch_and_run.ps1

# Function to run the project
function Run-Project {
    try {
        cargo run
        if ($LASTEXITCODE -ne 0) {
            throw "Cargo run failed with exit code $LASTEXITCODE"
        }
    } catch {
        Write-Error $_
        exit 1
    }
}

# Run the project (which will compile Slint files through build.rs)
Run-Project