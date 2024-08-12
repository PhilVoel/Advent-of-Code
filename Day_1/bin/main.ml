let second_day = true

let rec get_nums line i =
    try match Char.code line.[i] - Char.code '0' with
    c when c >= 0 && c<=9 -> c :: get_nums line (i+1)
    | _ -> get_nums line (i+1)
    with _ -> []

let rec sum list =
    match list with
    x :: rest -> x + sum rest
    | [] -> 0

let rec replace line num =
    if num > 9 then
        line
    else
        let (regex_string, replacement) = List.nth [
            ("one", "o1e");
            ("two", "t2o");
            ("three", "t3e");
            ("four", "f4r");
            ("five", "f5e");
            ("six", "s6x");
            ("seven", "s7n");
            ("eight", "e8t");
            ("nine", "n9e");
        ] (num-1)
        in
        let regex = Str.regexp regex_string
        in
        Str.global_replace regex replacement (replace line (num+1))

let convert_words line = 
    if second_day then
        replace line 1
    else
        line

let () = In_channel.with_open_text "input" In_channel.input_lines |> List.map (fun line -> get_nums (convert_words line) 0) |> List.map (fun l -> 10 * List.nth l 0 + List.nth l (List.length l - 1))  |> sum |> Printf.printf "%i\n" 
