inotifywait -r -m -e modify ./demo/src | 
   while read file_path file_event file_name; do 
       echo ${file_path}${file_name} event: ${file_event}
   done
