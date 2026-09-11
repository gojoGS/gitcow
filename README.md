# git COW - lean worktree-like experimental project

## Intended usage:

```bash
# We are in the ~/Projects/gitcow/ directory
# The directory is empty
#
# gitcow/
#

git cow init <git repo url>
# the above command created the etalon directory inside the project dir.
# gitcow/
#     etalon/
#         README.md
#         LICENSE
#

git cow create <new feature branch name>
# by running this command, an overlay directory is created. after
# creation, its contents match the etalon to the observer,
# but at the filesystem level, no duplicate files are persisted.
#
# gitcow/
#     etalon/
#         README.md
#         LICENSE
#     feature-branch/

cd feature-branch
echo "Contributors get apply pie." >> README.md
# now, that we've made a modification to a file from the etalon,
# a copy of the README.md file is created in feature-branch/ and gets updated. 
#
# gitcow/
#     etalon/
#         README.md
#         LICENSE
#     feature-branch/
#         README.md
 
```
