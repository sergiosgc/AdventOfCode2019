import sugar
import sequtils
import strutils
import system

proc seqDeltas(s: seq[int]): seq[int] =
    var res: seq[int]
    for index in 0..(s.len-1):
        if index > 0:
            res.add(s[index] - s[index-1])
    return res

var input =
    stdin
    .lines
    .toSeq
    .map( line => line
                  .split(" ")
                  .map(part => part.strip())
                  .filter( part => part != "")
                  .map(parseInt)
    )
echo(
    input
    .map( seqDeltas )
    .map( deltaRow => (
        deltaRow
        .map( delta => (delta, abs(delta) > 0 and abs(delta) < 4))
        .foldl( (b[0], a[1] and b[1] and a[0]/abs(a[0]) == b[0]/abs(b[0]) ) )
    ))
    .filter( a => a[1] )
    .len()
)
