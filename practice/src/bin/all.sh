#!/bin/sh

#./all.sh /home/positoro/atcoder/target/debug/practice-b
a=0
while [ $a -lt 120 ]
do
    #echo $a
    cat ./gomipipe | tee -a A.log | python3 ./B_question.py $a -s | tee -a Q.log | $1 > ./gomipipe
    a=`expr $a + 1`
done
