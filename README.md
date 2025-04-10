# righthook

This is an experiment of rewriting the [lefthook](https://github.com/evilmartians/lefthook) in Rust.

To do

- [x] Basic run hooks
- [x] Install hooks to .git/hooks
- [x] Uninstall righthook hooks
- [x] Handle templates like {stage_files} and {push_files}
- [ ] Install only hooks supported by Git
- [ ] Run jobs in parallel
- [ ] Skip, glob, exclude options
- [ ] Add auto stage feature (including hiding unstaged changes)
- [ ] Add nested jobs
- [ ] Handle long lists of files with separate execution
