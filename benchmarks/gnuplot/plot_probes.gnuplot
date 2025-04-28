set terminal pngcairo size 800,600 enhanced font "Helvetica,10"
set output "insert_probes.png"

set datafile separator ","
set xlabel "Load Factor"
set ylabel "Probes per Insertion"
set grid

plot "insert_probes.csv" using 1:2 with linespoints title 'Greedy Insert Probes'
