{-# LANGUAGE CPP #-}
import System.Environment (getArgs)
import Solution

main :: IO ()
main = do
    args <- getArgs
    input <- readFile $ "inputs/" ++ DAY ++ ".txt"
    print $
        if "--part2" `notElem` args then
            commonPost . part1 . commonPre $ lines input
        else
            commonPost . part2 . commonPre $ lines input
