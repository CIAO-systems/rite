#!/bin/bash
PROTO_ROOT=proto
C_WHITE='\e[1;37m'
C_NORMAL='\e[0m'

echo "Generated from $PROTO_ROOT..."
PROTO_CMD="protoc --proto_path=$PROTO_ROOT "
# Optionally add Elixir output
if [[ ${ENABLE_ELIXIR:-true}  == "true" ]]; then
	echo -e " ...with "$C_WHITE"Elixir"$C_NORMAL" output"
	mkdir -p target/elixir
	PROTO_CMD+='--elixir_out=gen_descriptors=true,plugins=grpc:target/elixir '
	PROTO_CMD+='--elixir_opt=package_prefix=ciao_grpc '
fi
# Optionally add Java output
if [[ ${ENABLE_JAVA:-true} == "true" ]]; then
	echo -e " ...with "$C_WHITE"Java"$C_NORMAL" output"
	mkdir -p target/java
	PROTO_CMD+='--java_out=target/java '
fi
# Optionally add Dart output
if [[ ${ENABLE_DART:-true}  == "true" ]]; then
	echo -e " ...with "$C_WHITE"Dart"$C_NORMAL" output"
	mkdir -p target/dart
	PROTO_CMD+='--dart_out=grpc:target/dart '
fi
# Optionally add Rust output
if [[ ${ENABLE_RUST:-true}  == "true" ]]; then
	echo -e " ...with "$C_WHITE"Rust"$C_NORMAL" output"
	mkdir -p target/rust
	PROTO_CMD+='--rust_out=experimental-codegen=enabled,kernel=cpp:target/rust '
fi

# Add all source files, that should be generated
PROTO_CMD+='google/protobuf/timestamp.proto'
PROTO_CMD+=' google/protobuf/duration.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/common/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/accounts/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/core/auth/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/core/config/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/devices/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/time_tracking/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/time_tracking/time_type/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/time_tracking/time_type/group/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/time_tracking/absences/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/time_tracking/cost_center/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/time_tracking/project/*.proto'
PROTO_CMD+=' $PROTO_ROOT/ciao/time_tracking/project/task/*.proto'

eval "$PROTO_CMD"
