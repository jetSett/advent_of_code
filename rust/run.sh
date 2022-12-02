# You should run that inside the rust/<year> folder

year=2022

day_number=$1
day=$(printf "day_%d" "${day_number}")
echo "Running ${day}"

cat ../../inputs/${year}/input-${day_number}.txt | cargo run --bin "${day}"
