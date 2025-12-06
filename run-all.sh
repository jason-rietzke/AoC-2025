# make sure dependencies are installed - bc for time calculation
if ! command -v bc &> /dev/null
then
	# install bc based on OS (also check what linux package manager is available)
	if [[ "$OSTYPE" == "linux-gnu"* ]]; then
		if command -v apt-get &> /dev/null; then
			sudo apt-get update
			sudo apt-get install -y bc
		elif command -v yum &> /dev/null; then
			sudo yum install -y bc
		elif command -v pacman &> /dev/null; then
			sudo pacman -S bc
		else
			echo "Unsupported Linux package manager. Please install 'bc' manually."
			exit 1
		fi
	elif [[ "$OSTYPE" == "darwin"* ]]; then
		# macOS
		if command -v brew &> /dev/null; then
			brew install bc
		else
			echo "Homebrew is not installed. Please install Homebrew and then install 'bc' manually."
			exit 1
		fi
	else
		echo "Unsupported OS. Please install 'bc' manually."
		exit 1
	fi
fi

# Collect system information
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
	CPU_MODEL=$(lscpu | grep "Model name" | sed 's/Model name:[[:space:]]*//')
	CPU_CORES=$(lscpu | grep "^CPU(s):" | awk '{print $2}')
	OS_INFO=$(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)
elif [[ "$OSTYPE" == "darwin"* ]]; then
	CPU_MODEL=$(sysctl -n machdep.cpu.brand_string)
	CPU_CORES=$(sysctl -n hw.ncpu)
	OS_INFO=$(sw_vers -productName)" "$(sw_vers -productVersion)
else
	CPU_MODEL="Unknown"
	CPU_CORES="Unknown"
	OS_INFO="Unknown"
fi

days=()
declare -A day_outputs
declare -A day_times

for dir in day*/ ; do
	day=$(basename "$dir")
	days+=("$day")
done

# sort days array
IFS=$'\n' sorted_days=($(sort <<<"${days[*]}"))
unset IFS

for day in "${sorted_days[@]}" ; do
	cargo build -p "$day" --release
done

clear

all_start_time=$(date +%s.%N)

declare -A day_outputs
declare -A day_times

for day in "${sorted_days[@]}" ; do
	echo "=== Running $day ==="
	start_time=$(date +%s.%N)
	day_output=$(./target/release/"$day" 2>&1)
	echo "$day_output"
	end_time=$(date +%s.%N)
	elapsed_time=$(echo "($end_time - $start_time) * 1000000" | bc)
	
	# Store result for README using associative arrays
	day_outputs["$day"]="$day_output"
	day_times["$day"]="$elapsed_time"
	
	# format nicely to microseconds, milliseconds or seconds
	if (( $(echo "$elapsed_time < 1000" | bc -l) )); then
		formatted_time="${elapsed_time} µs"
		echo "=== $day took $formatted_time ==="
	elif (( $(echo "$elapsed_time < 1000000" | bc -l) )); then
		elapsed_time_ms=$(echo "scale=3; $elapsed_time / 1000" | bc)
		formatted_time="${elapsed_time_ms} ms"
		echo "=== $day took $formatted_time ==="
	else
		elapsed_time_s=$(echo "scale=3; $elapsed_time / 1000000" | bc)
		formatted_time="${elapsed_time_s} s"
		echo "=== $day took $formatted_time ==="
	fi
	echo ""
done

all_end_time=$(date +%s.%N)
all_elapsed_time=$(echo "($all_end_time - $all_start_time) * 1000000" | bc)
if (( $(echo "$all_elapsed_time < 1000" | bc -l) )); then
	total_time="${all_elapsed_time} µs"
	echo "All days took $total_time"
elif (( $(echo "$all_elapsed_time < 1000000" | bc -l) )); then
	all_elapsed_time_ms=$(echo "scale=3; $all_elapsed_time / 1000" | bc)
	total_time="${all_elapsed_time_ms} ms"
	echo "All days took $total_time"
else
	all_elapsed_time_s=$(echo "scale=3; $all_elapsed_time / 1000000" | bc)
	total_time="${all_elapsed_time_s} s"
	echo "All days took $total_time"
fi

# Generate README.md
cat > README.md << 'EOF'
# 🎄 Advent of Code 2025 🎄

Welcome to my Advent of Code 2025 solutions! This repository contains my attempts at solving the daily programming puzzles from [Advent of Code 2025](https://adventofcode.com/2025).

## 📊 System Specifications

EOF

echo "- **CPU:** $CPU_MODEL" >> README.md
echo "- **Cores:** $CPU_CORES" >> README.md
echo "- **OS:** $OS_INFO" >> README.md
echo "" >> README.md

cat >> README.md << 'EOF'
## 🏃 Performance Results

All solutions are implemented in Rust and compiled with `--release` optimizations.

EOF

# Add individual day results
for day in "${sorted_days[@]}"; do
	elapsed_us="${day_times[$day]}"
	output="${day_outputs[$day]}"
	
	# Format time
	if (( $(echo "$elapsed_us < 1000" | bc -l) )); then
		time_str="${elapsed_us} µs"
	elif (( $(echo "$elapsed_us < 1000000" | bc -l) )); then
		time_ms=$(echo "scale=3; $elapsed_us / 1000" | bc)
		time_str="${time_ms} ms"
	else
		time_s=$(echo "scale=3; $elapsed_us / 1000000" | bc)
		time_str="${time_s} s"
	fi
	
	# Clean up day name for display
	day_num=$(echo "$day" | sed 's/day0*/Day /')
	
	echo "### $day_num" >> README.md
	echo "" >> README.md
	echo "\`\`\`" >> README.md
	echo "$output" >> README.md
	echo "\`\`\`" >> README.md
	echo "" >> README.md
	echo "*Total time: $time_str*" >> README.md
	echo "" >> README.md
done

cat >> README.md << EOF

---

**Total Runtime:** $total_time

## 🛠️ Building and Running

To build all solutions:
\`\`\`bash
cargo build --release
\`\`\`

To run all days:
\`\`\`bash
./run-all.sh
\`\`\`

To run a specific day:
\`\`\`bash
cargo run -p day01 --release
\`\`\`

## 📝 Notes

Each day's solution is organized in its own workspace member with:
- \`src/main.rs\` - The solution code
- \`data.txt\` - The puzzle input
- \`Cargo.toml\` - Day-specific dependencies

---

*Last updated: $(date '+%Y-%m-%d %H:%M:%S')*
EOF

echo ""
echo "✅ README.md generated successfully!"
