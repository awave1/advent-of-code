let get_input_file day part is_test =
  "input/"
  ^ (if int_of_string day < 9 then "day0" else "day")
  ^ day ^ "/" ^ "p" ^ part
  ^ if is_test then ".test" else ""

let safe_get_element array index default =
  if index >= 0 && index < Array.length array then array.(index) else default

let is_digit c =
  let ascii_val = Char.code c in
  ascii_val >= 48 && ascii_val <= 57

let is_int s =
  try
    let _ = int_of_string s in
    true
  with Failure _ -> false

(* l m a o *)
let get_two_digit_str str =
  match str with
  | str when String.length str > 2 ->
      String.make 1 str.[0] ^ String.make 1 str.[String.length str - 1]
  | str when String.length str = 1 -> str ^ str
  | _ -> str

let solve_p1 day part is_test =
  let input_path = get_input_file day part is_test in
  let problem_input = input_path |> open_in in

  let rec read_lines result =
    try
      let line = input_line problem_input in

      (* Converting string to individual chars *)
      let chars = line |> String.to_seq |> List.of_seq in
      let nums_only = List.filter is_digit chars in
      let strings = List.map (String.make 1) nums_only in
      let joined = String.concat "" strings in

      (* let result = int_of_string (get_two_digit_str joined) in *)
      read_lines result + int_of_string (get_two_digit_str joined)
    with End_of_file -> result
  in

  let sum = read_lines 0 in
  close_in problem_input;

  sum

let rec match_start list_of_chars =
  match list_of_chars with
  | [] -> ""
  | c :: cs ->
      if is_digit c then String.make 1 c
      else if
        String.starts_with ~prefix:"one"
          (c :: cs |> List.to_seq |> String.of_seq)
      then "1"
      else if
        String.starts_with ~prefix:"two"
          (c :: cs |> List.to_seq |> String.of_seq)
      then "2"
      else if
        String.starts_with ~prefix:"three"
          (c :: cs |> List.to_seq |> String.of_seq)
      then "3"
      else if
        String.starts_with ~prefix:"four"
          (c :: cs |> List.to_seq |> String.of_seq)
      then "4"
      else if
        String.starts_with ~prefix:"five"
          (c :: cs |> List.to_seq |> String.of_seq)
      then "5"
      else if
        String.starts_with ~prefix:"six"
          (c :: cs |> List.to_seq |> String.of_seq)
      then "6"
      else if
        String.starts_with ~prefix:"seven"
          (c :: cs |> List.to_seq |> String.of_seq)
      then "7"
      else if
        String.starts_with ~prefix:"eight"
          (c :: cs |> List.to_seq |> String.of_seq)
      then "8"
      else if
        String.starts_with ~prefix:"nine"
          (c :: cs |> List.to_seq |> String.of_seq)
      then "9"
      else match_start cs

let reverse str =
  str |> String.to_seq |> List.of_seq |> List.rev |> List.to_seq
  |> String.of_seq

let rec match_end list_of_chars =
  match list_of_chars with
  | [] -> ""
  | c :: cs ->
      let res = match_end cs in

      let str_tail = c :: cs |> List.to_seq |> String.of_seq in

      if is_int res then res
      else if is_digit c then String.make 1 c
      else if String.starts_with ~prefix:"one" str_tail then "1"
      else if String.starts_with ~prefix:"two" str_tail then "2"
      else if String.starts_with ~prefix:"three" str_tail then "3"
      else if String.starts_with ~prefix:"four" str_tail then "4"
      else if String.starts_with ~prefix:"five" str_tail then "5"
      else if String.starts_with ~prefix:"six" str_tail then "6"
      else if String.starts_with ~prefix:"seven" str_tail then "7"
      else if String.starts_with ~prefix:"eight" str_tail then "8"
      else if String.starts_with ~prefix:"nine" str_tail then "9"
      else res

let solve_p2 day part is_test =
  let input_path = get_input_file day part is_test in
  let problem_input = input_path |> open_in in

  let rec read_lines result =
    try
      let line = input_line problem_input in

      let chars = line |> String.to_seq |> List.of_seq in
      let start = match_start chars in
      let end' = match_end chars in
      let result_num = start ^ end' |> int_of_string in

      read_lines result + result_num
    with End_of_file -> result
  in

  let sum = read_lines 0 in

  close_in problem_input;

  sum

let () =
  let is_test = bool_of_string (safe_get_element Sys.argv 1 "false") in

  Printf.printf "d1, p1: %i\n" (solve_p1 "1" "1" is_test);
  Printf.printf "d1, p2: %i\n" (solve_p2 "1" "2" is_test)
