for i in `seq 518100 10 520000`; do
  echo -n $i ""
  perf stat -B -e page-faults,minor-faults,cache-references,cache-misses,cycles,instructions,branches,branch-misses ./target/release/vec_medley vec 100 $i 2>&1 | grep -E 'page-faults' | awk '{print $1;}' | tr -d ',' | tr '\n' ' '
  echo 
done

