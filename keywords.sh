#!/usr/bin/env bash

# {{{
read -d '' keywords <<-EOF
accept_on               style_keyword_1space
alias                   style_keyword_1space
always                  style_keyword_1space
always_comb             style_keyword_1space
always_ff               style_keyword_1space
always_latch            style_keyword_1space
and                     style_keyword_1space
assert                  style_keyword_1space
assign                  style_keyword_1space
assume                  style_keyword_1space
automatic               style_keyword_1space
before                  style_keyword_1space
begin                   style_keyword_namedblock_0space
bind                    style_keyword_1space
bins                    style_keyword_1space
binsof                  style_keyword_1space
bit                     style_keyword_1space
break                   style_keyword_0space
buf                     style_keyword_1space
bufif0                  style_keyword_1space
bufif1                  style_keyword_1space
byte                    style_keyword_1space
case                    style_keyword_1space
casex                   style_keyword_1space
casez                   style_keyword_1space
cell                    style_keyword_1space
chandle                 style_keyword_1space
checker                 style_keyword_1space
class                   style_keyword_1space
clocking                style_keyword_1space
cmos                    style_keyword_1space
config                  style_keyword_1space
const                   style_keyword_1space
constraint              style_keyword_1space
context                 style_keyword_1space
continue                style_keyword_0space
cover                   style_keyword_1space
covergroup              style_keyword_1space
coverpoint              style_keyword_1space
cross                   style_keyword_1space
deassign                style_keyword_1space
default                 style_keyword_0space
defparam                style_keyword_1space
design                  style_keyword_1space
disable                 style_keyword_1space
dist                    style_keyword_1space
do                      style_keyword_1space
edge                    style_keyword_1space
else                    style_keyword_block_0space
end                     style_keyword_namedblock_0space
endcase                 style_keyword_block_0space
endchecker              style_keyword_namedblock_0space
endclass                style_keyword_namedblock_0space
endclocking             style_keyword_namedblock_0space
endconfig               style_keyword_namedblock_0space
endfunction             style_keyword_namedblock_0space
endgenerate             style_keyword_block_0space
endgroup                style_keyword_namedblock_0space
endinterface            style_keyword_namedblock_0space
endmodule               style_keyword_namedblock_0space
endpackage              style_keyword_namedblock_0space
endprimitive            style_keyword_namedblock_0space
endprogram              style_keyword_namedblock_0space
endproperty             style_keyword_namedblock_0space
endspecify              style_keyword_block_0space
endsequence             style_keyword_namedblock_0space
endtable                style_keyword_block_0space
endtask                 style_keyword_namedblock_0space
enum                    style_keyword_1space
event                   style_keyword_1space
eventually              style_keyword_1space
expect                  style_keyword_1space
export                  style_keyword_1space
extends                 style_keyword_1space
extern                  style_keyword_1space
final                   style_keyword_block_0space
first_match             style_keyword_1space
for                     style_keyword_1space
force                   style_keyword_1space
foreach                 style_keyword_1space
forever                 style_keyword_1space
fork                    style_keyword_block_0space
forkjoin                style_keyword_1space
function                style_keyword_1space
generate                style_keyword_block_0space
genvar                  style_keyword_1space
global                  style_keyword_1space
highz0                  style_keyword_1space
highz1                  style_keyword_1space
if                      style_keyword_1space
iff                     style_keyword_1space
ifnone                  style_keyword_1space
ignore_bins             style_keyword_1space
illegal_bins            style_keyword_1space
implements              style_keyword_1space
implies                 style_keyword_1space
import                  style_keyword_1space
incdir                  style_keyword_1space
include                 style_keyword_1space
initial                 style_keyword_block_0space
inout                   style_keyword_1or2space
input                   style_keyword_1or2space
inside                  style_keyword_1space
instance                style_keyword_1space
int                     style_keyword_1space
integer                 style_keyword_1space
interconnect            style_keyword_1space
interface               style_keyword_1space
intersect               style_keyword_1space
join                    style_keyword_block_0space
join_any                style_keyword_block_0space
join_none               style_keyword_block_0space
large                   style_keyword_1space
let                     style_keyword_1space
liblist                 style_keyword_1space
library                 style_keyword_1space
local                   style_keyword_1space
localparam              style_keyword_1space
logic                   style_keyword_1space
longint                 style_keyword_1space
macromodule             style_keyword_1space
matches                 style_keyword_1space
medium                  style_keyword_1space
modport                 style_keyword_1space
module                  style_keyword_1space
nand                    style_keyword_1space
negedge                 style_keyword_1space
nettype                 style_keyword_1space
new                     style_keyword_0space
nexttime                style_keyword_1space
nmos                    style_keyword_1space
nor                     style_keyword_1space
noshowcancelled         style_keyword_1space
not                     style_keyword_1space
notif0                  style_keyword_1space
notif1                  style_keyword_1space
null                    style_keyword_0space
or                      style_keyword_1space
output                  style_keyword_1space
package                 style_keyword_1space
packed                  style_keyword_1space
parameter               style_keyword_1space
pmos                    style_keyword_1space
posedge                 style_keyword_1space
primitive               style_keyword_1space
priority                style_keyword_1space
program                 style_keyword_1space
property                style_keyword_1space
protected               style_keyword_1space
pull0                   style_keyword_1space
pull1                   style_keyword_1space
pulldown                style_keyword_1space
pullup                  style_keyword_1space
pulsestyle_ondetect     style_keyword_1space
pulsestyle_onevent      style_keyword_1space
pure                    style_keyword_1space
rand                    style_keyword_1space
randc                   style_keyword_1space
randcase                style_keyword_1space
randsequence            style_keyword_1space
rcmos                   style_keyword_1space
real                    style_keyword_1space
realtime                style_keyword_1space
ref                     style_keyword_1space
reg                     style_keyword_1space
reject_on               style_keyword_1space
release                 style_keyword_1space
repeat                  style_keyword_1space
restrict                style_keyword_1space
return                  style_keyword_0space
rnmos                   style_keyword_1space
rpmos                   style_keyword_1space
rtran                   style_keyword_1space
rtranif0                style_keyword_1space
rtranif1                style_keyword_1space
s_always                style_keyword_1space
s_eventually            style_keyword_1space
s_nexttime              style_keyword_1space
s_until                 style_keyword_1space
s_until_with            style_keyword_1space
scalared                style_keyword_1space
sequence                style_keyword_1space
shortint                style_keyword_1space
shortreal               style_keyword_1space
showcancelled           style_keyword_1space
signed                  style_keyword_1space
small                   style_keyword_1space
soft                    style_keyword_1space
solve                   style_keyword_1space
specify                 style_keyword_block_0space
specparam               style_keyword_1space
static                  style_keyword_1space
string                  style_keyword_1space
strong                  style_keyword_1space
strong0                 style_keyword_1space
strong1                 style_keyword_1space
struct                  style_keyword_1space
super                   style_keyword_0space
supply0                 style_keyword_1space
supply1                 style_keyword_1space
sync_accept_on          style_keyword_1space
sync_reject_on          style_keyword_1space
table                   style_keyword_block_0space
tagged                  style_keyword_1space
task                    style_keyword_1space
this                    style_keyword_0space
throughout              style_keyword_1space
time                    style_keyword_1space
timeprecision           style_keyword_1space
timeunit                style_keyword_1space
tran                    style_keyword_1space
tranif0                 style_keyword_1space
tranif1                 style_keyword_1space
tri                     style_keyword_1space
tri0                    style_keyword_1space
tri1                    style_keyword_1space
triand                  style_keyword_1space
trior                   style_keyword_1space
trireg                  style_keyword_1space
type                    style_keyword_1space
typedef                 style_keyword_1space
union                   style_keyword_1space
unique                  style_keyword_1space
unique0                 style_keyword_1space
unsigned                style_keyword_1space
until                   style_keyword_1space
until_with              style_keyword_1space
untyped                 style_keyword_1space
use                     style_keyword_1space
uwire                   style_keyword_1space
var                     style_keyword_1space
vectored                style_keyword_1space
virtual                 style_keyword_1space
void                    style_keyword_1space
wait                    style_keyword_1space
wait_order              style_keyword_1space
wand                    style_keyword_1space
weak                    style_keyword_1space
weak0                   style_keyword_1space
weak1                   style_keyword_1space
while                   style_keyword_1space
wildcard                style_keyword_1space
wire                    style_keyword_1space
with                    style_keyword_1space
within                  style_keyword_1space
wor                     style_keyword_1space
xnor                    style_keyword_1space
xor                     style_keyword_1space
EOF
# }}}

styles=$(echo "$keywords" | \
         grep style_keyword_ | \
         tr -s ' ' | \
         cut -d ' ' -f 2 | \
         sort -u)

for s in $styles; do
  kws=$(echo "$keywords" | grep $s | tr -s ' ' | cut -d ' ' -f 1);
  RS=$(
    i=0;
    for kw in $kws; do
      if [ $((i++)) -eq 0 ]; then
        echo "[ \"$kw\"";
      else
        echo ", \"$kw\"";
      fi
    done
    echo '].join("|");';
  )
  echo "$RS" > $s.keywords.rs
done
