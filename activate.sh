#!/bin/bash

TARGET_PATH=$(pwd)/engine/target/release

if ![ -f "${TARGET_PATH}/level_editor" ]; then
	echo "Building level editor"
	(cd engine && cargo build --release --example level_editor)
else
	echo "Level editor built"
fi

if echo "${PATH}" | grep "${TARGET_PATH}"; then
	echo "Adding ${TARGET_PATH} to path"
	export PATH=${TARGET_PATH}:${PATH}
fi

