open String
open List

let part_two = true

type count = {
    r: int;
    g: int;
    b: int;
}

let max_allowed = {
    r = 12;
    g = 13;
    b = 14;
}

let max prev (c, t) =
    match t with
    "red" -> {r=Int.max prev.r c; g=prev.g; b=prev.b}
    |"green" -> {r=prev.r; g=Int.max prev.g c; b=prev.b}
    |"blue" -> {r=prev.r; g=prev.g; b=Int.max prev.b c}
    |_ -> raise Exit

let part_one_or_two l =
    if part_two then
        map (fun (_, count) -> count.r * count.g * count.b) l
    else
        filter_map (fun (i, count) -> if count.r <= max_allowed.r && count.g <= max_allowed.g && count.b <= max_allowed.b then Some i else None) l

let _ = In_channel.with_open_text "input" In_channel.input_lines |> mapi (fun i e -> (i+1, split_on_char ':' e |> fun l -> nth l 1  |> split_on_char ',' |> map (split_on_char ';') |> flatten |> map (fun l -> let s = trim l |> split_on_char ' ' in (nth s 0 |> int_of_string, nth s 1)) |> fold_left max {r = 0; g=0; b=0})) |> part_one_or_two |> fold_left Int.add 0 |> Printf.printf "\n%i\n"
