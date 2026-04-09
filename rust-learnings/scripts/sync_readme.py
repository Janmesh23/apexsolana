import os
import re
import requests
import sys

# Configuration
REPO = os.getenv("GITHUB_REPOSITORY", "Janmesh23/apexrust")
TOKEN = os.getenv("GITHUB_TOKEN")
README_PATH = "rust-learnings/README.md"

def fetch_github_data():
    headers = {"Authorization": f"token {TOKEN}"} if TOKEN else {}
    
    # Fetch Issues
    issues_url = f"https://api.github.com/repos/{REPO}/issues?state=all&per_page=100"
    response = requests.get(issues_url, headers=headers)
    if response.status_code != 200:
        print(f"Error fetching issues: {response.status_code} {response.text}")
        return [], []
    
    all_items = response.json()
    issues = [i for i in all_items if "pull_request" not in i]
    prs = [i for i in all_items if "pull_request" in i]
    
    return issues, prs

def get_status_for_issue(issue_num_str, issues, prs):
    # Pattern to match #XX in title
    target = f"#{issue_num_str}"
    
    # 1. Check for PRs first (Merge status is highest priority)
    for pr in prs:
        if target in pr["title"]:
            if pr["state"] == "closed":
                return "🎉 Merged" # Simplified, could check 'merged_at' if we wanted
            if pr["state"] == "open":
                return f"🛠️ PR Open (@{pr['user']['login']})"

    # 2. Check for Issues
    for issue in issues:
        if target in issue["title"]:
            if issue["state"] == "closed":
                return "✅ Completed"
            if issue.get("assignees"):
                assignee = issue["assignees"][0]["login"]
                return f"👤 Assigned to @{assignee}"
            return "🟢 Open"
            
    return "🟢 Open" # Default

def update_readme(issues, prs):
    if not os.path.exists(README_PATH):
        print(f"README not found at {README_PATH}")
        return

    with open(README_PATH, "r") as f:
        lines = f.readlines()

    updated_lines = []
    # Regex to catch: | #04 | [Title](link) | Concept | Diff | Status |
    # We want to keep everything up to the 4th pipe and replace the content after it.
    row_pattern = re.compile(r"\| (#\d{2}) \| (.*?) \| (.*?) \| (.*?) \| (.*?) \|")

    for line in lines:
        match = row_pattern.search(line)
        if match:
            issue_num = match.group(1).replace("#", "")
            new_status = get_status_for_issue(issue_num, issues, prs)
            
            # Construct the new line
            # groups: 1=#04, 2=[Title](link), 3=Concept, 4=Diff, 5=Status
            new_line = f"| {match.group(1)} | {match.group(2)} | {match.group(3)} | {match.group(4)} | {new_status} |\n"
            updated_lines.append(new_line)
        else:
            updated_lines.append(line)

    with open(README_PATH, "w") as f:
        f.writelines(updated_lines)

if __name__ == "__main__":
    if not TOKEN:
        print("GITHUB_TOKEN not set, running in read-only mode (using public API limits)")
    
    issues, prs = fetch_github_data()
    update_readme(issues, prs)
    print("README updated successfully.")
