
# build authorization plugin
bap:
	cd .. && mkdir build && bazel build //plugins/Authorization:authorization 

# bazel clean
bc:
	cd .. && bazel clean

# bazel clean async
bca:
	cd .. && bazel clean --async

# bazel test alicedb
bta:
	cd .. && bazel test //Plugins/AliceDatabase:rs_test

# bazel build alicedb
bba:
	cd .. && CARGO_BAZEL_REPIN=true bazel build //Plugins/AliceDatabase:rs_bazel

# enter to nix
etn:
	nix-shell shell.nix