import os
import subprocess

def search_arti_source(query, path="arti-upstream"):
    """
    Scans the Arti source for specific Proposal 340 keywords or 
    circuit-build hooks using ripgrep (rg) for maximum speed on M5.
    """
    # Focusing on specific crates related to Proposal 340/CGO
    search_targets = [
        f"{path}/crates/tor-proto",
        f"{path}/crates/tor-circmgr",
        f"{path}/crates/tor-guardmgr"
    ]
    
    # Executing search for CGO and Circuit Build events
    cmd = ["rg", "-n", "-C", "3", query] + search_targets
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.stdout

# Example usage for the agent:
# search_arti_source("Prop340") or search_arti_source("CircuitBuild")

