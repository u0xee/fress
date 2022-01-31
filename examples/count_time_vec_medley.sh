function medley () {
  for i in `seq $1 $2 $3`; do
    #echo -n $i ""
    perf stat ./target/release/vec_medley vector-medley 0 $i 2 3 \
    2>&1 | grep -E 'elapsed' | awk '{print $1;}' | tr -d ',' | tr '\n' ' '
    echo 
  done
}
    #2>&1 | grep -E 'page-faults' | awk '{print $1;}' | tr -d ',' | tr '\n' ' '

#medley 20 20 1000
#medley 1050 50 5000
#medley 5100 100 10000

#medley 2000 4000 100000
#medley 100000 50000 1000000

function medley_history () {
  echo "Size $1"
  for i in 1 2 3 4 5 8 10 15 20 35 50 100 200 500 1000 2000 5000 10000 20000 50000 100000 200000 500000 1000000 2000000 5000000; do
    #echo -n $i ""
    perf stat ./target/release/vec_medley vector-history 8 $1 $i $2 \
    2>&1 | grep -E 'elapsed' | awk '{print $1;}' | tr -d ',' | tr '\n' ' '
    echo 
  done
}

medley_history $1 $2 | tee test.txt

