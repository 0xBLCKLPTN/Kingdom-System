
# build authorization plugin
bap:
	mkdir build && bazel build //plugins/Authorization:authorization 

# bazel clean
bc:
	bazel clean

# bazel clean async
bca:
	bazel clean --async

# enter to nix
etn:
	nix-shell shell.nix