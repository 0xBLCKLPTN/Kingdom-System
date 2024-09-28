
# build authorization plugin
bap:
	mkdir build && bazel build //plugins/Authorization:authorization 

# bazel clean
bc:
	bazel clean

# bazel clean async
bca:
	bazel clean --async

# bazel test alicedb
bta:
	bazel test //Plugins/AliceDatabase:rs_test

# bazel build alicedb
bba:
	CARGO_BAZEL_REPIN=true bazel build //Plugins/AliceDatabase:rs_bazel

# enter to nix
etn:
	nix-shell shell.nix