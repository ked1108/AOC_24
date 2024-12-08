#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 <project_name>"
  exit 1
fi

project_name=$1

cargo new "$project_name"

if [ $? -ne 0 ]; then
  echo "Failed to create Cargo project."
  exit 1
fi

template_dir="./template"
target_dir="./$project_name/src"

if [ -f "$template_dir/main.rs" ]; then
  cp "$template_dir/main.rs" "$target_dir/main.rs"
	sed -i "s|use dayX::solution;|use ${project_name}::solution;|g" "$target_dir/main.rs"
else
  echo "main.rs not found in $template_dir"
fi

if [ -f "$template_dir/lib.rs" ]; then
  cp "$template_dir/lib.rs" "$target_dir/lib.rs"
else
  echo "lib.rs not found in $template_dir"
fi

echo "Cargo project '$project_name' created and files copied from template directory."

