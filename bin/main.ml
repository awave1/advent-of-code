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

module NumMap = Map.Make (String)

let build_map =
  let num_map = NumMap.empty in
  let num_map = NumMap.add "one" 1 num_map in
  let num_map = NumMap.add "two" 2 num_map in
  let num_map = NumMap.add "three" 3 num_map in
  let num_map = NumMap.add "four" 4 num_map in
  let num_map = NumMap.add "five" 5 num_map in
  let num_map = NumMap.add "six" 6 num_map in
  let num_map = NumMap.add "seven" 7 num_map in
  let num_map = NumMap.add "eight" 8 num_map in
  let num_map = NumMap.add "nine" 9 num_map in

  num_map

let solve_p2 day part is_test =
  let input_path = get_input_file day part is_test in
  let problem_input = input_path |> open_in in
  let num_map = build_map in

  let str_digit_regex =
    Str.regexp
      "\\(one\\|two\\|three\\|four\\|five\\|six\\|seven\\|eight\\|nine\\)"
  in

  let rec iterate_matches text pos =
    try
      let found_pos = Str.search_forward str_digit_regex text pos in
      let matched = Str.matched_string text in

      (* Printf.printf "in: %s\n" text; *)
      (* Printf.printf "Match found: %s\n" matched; *)
      let regex = Str.regexp matched in
      let replaced =
        Str.replace_first regex
          (NumMap.find matched num_map |> string_of_int)
          text
      in

      (* Printf.printf "replaced: %s\n" replaced; *)
      iterate_matches replaced (found_pos + 1)
    with Not_found -> text
  in

  let rec read_lines result =
    try
      let line = input_line problem_input in
      let converted = iterate_matches line 0 in

      (* Converting string to individual chars *)
      let chars = converted |> String.to_seq |> List.of_seq in
      let nums_only = List.filter is_digit chars in
      let strings = List.map (String.make 1) nums_only in
      let joined = String.concat "" strings in

      read_lines result + int_of_string (get_two_digit_str joined)
    with End_of_file -> result
  in

  let sum = read_lines 0 in

  close_in problem_input;

  sum

let () =
  if Array.length Sys.argv < 2 then prerr_endline "Specify at least day";

  let day = safe_get_element Sys.argv 1 "1" in
  let part = safe_get_element Sys.argv 2 "1" in
  let is_test = bool_of_string (safe_get_element Sys.argv 3 "true") in

  (* Printf.printf "d%s, p%s: %i\n" day part (solve day part is_test); *)
  Printf.printf "d%s, p%s: %i\n" day part (solve_p2 day part false)
