use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::HashMap, ops::Index};

lazy_static! {
    static ref CHAR_SCORE_MAP: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|f| (f.1, f.0 + 1))
        .collect();
}

fn main() {
    println!("{}", solve_part_1(EXAMPLE_INPUT));
    println!("{}", solve_part_1(ACTUAL_INPUT));
    println!("{}", solve_part_2(EXAMPLE_INPUT));
    println!("{}", solve_part_2(ACTUAL_INPUT));
}

fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2_usize))
        .filter_map(|compartments| {
            let alternate_list = compartments.1.chars().collect::<Vec<char>>();

            // Compute the character intersection.
            let common_types: Vec<usize> = compartments
                .0
                .chars()
                .into_iter()
                .filter_map(|compartment_item| {
                    if alternate_list.contains(&compartment_item) {
                        // If a character intersects then we return it's score.
                        Some(*CHAR_SCORE_MAP.index(&compartment_item))
                    } else {
                        // Otherwise we skip by way of filter_map continuing on
                        // none.
                        None
                    }
                })
                .collect();

            // Remove first to not assume 1 solution per rucksack.
            common_types.first().cloned()
        })
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .tuples::<(&str, &str, &str)>()
        .filter_map(|group| {
            let group_member_b = group.1.chars().collect::<Vec<char>>();
            let group_member_c = group.2.chars().collect::<Vec<char>>();

            // Compute the characters from group member a that are also present in b&c.
            let common_types: Vec<usize> = group
                .0
                .chars()
                .into_iter()
                .filter_map(|compartment_item| {
                    if group_member_b.contains(&compartment_item) && group_member_c.contains(&compartment_item) {
                        // If a character intersects then we return it's score.
                        Some(*CHAR_SCORE_MAP.index(&compartment_item))
                    } else {
                        // Otherwise we skip by way of filter_map continuing on
                        // none.
                        None
                    }
                })
                .collect();

            // Remove first to not assume 1 solution per group.
            common_types.first().cloned()
        }).sum()
}

const EXAMPLE_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

const ACTUAL_INPUT: &str = r#"CjhshBJCSrTTsLwqwqwb
GtmnFHlDfcpHbLZjtTTRLWwb
fDfNHHjVFNvvrvVBJJdS
PPWvWQjPhrPQwlMWJJdMDGbJTdCJ
rsqsStgNNggBNBZHSrJGdJdCFRRZCFbGbTdJ
qgBqqHzzggBpzSnBNqNSSSgcfhrVlVmwPljQVLVwVvQmmzVl
bBBGBfmGvBTnGtGJBtGpcJbZrrddjqrZhDldwdcqrjrjDr
HWPSQMsPHFsMWPVVMVSHCwDCDwwZZvwjwQZZwjdd
vVHPgHHFRLfpfJTLLtJL
LWLZhDBHhWWHjBwHwBjCTrBnnrQTQCJTJpTQBQ
vvdcqbRmvwSFmRqPFSqwdvtQnrpCQJpNNVnrptVCtCCP
wRSffqlFgvvdgdlzhLWWDzLljZhhGL
LNbTbPdTNgnShgSjmHcqtQGCtrctCPcQqc
vzWZDZZBlDwlzvDRZfFBRzVBtMMGHCcpjrqBGMtQQppqHtCq
DfRvFZjZRsRjlVWsjFlWVWvdSdbhsSmnSngTdTSTsJdSLm
nnZRbfZRTZfRsbZTFPRfpbRJdqqQNPwNqvvwvvvSwdQqdS
WjDzjMWMHpQwcSQWcJSS
hrHhHDgpphmjtMrGLDmGgmnbsflbfZCsnsClltsVsfRR
QFngsFnGdLGFGVRgLLqWPQPWvBrMDppCPrDB
NfcHZZzbHthSZtNtNfhHWhClDvCqlrqBvrCpWMWB
fSMSHjbTMVgVVngVgT
llnMffwbvCnffHvJJPJpPBNVVZDJDmmS
QssGGHhRgcqjRRTWGWRGDZcVFZBpNBmPFBDSmZNP
hzsRHWQshzgRjjsgQRTsbwzCtnrrlnrwzMCtvMff
HdddHHmtDMDTqHHSftmqdmfdssGzrsjVGtrllWlGZGsnlnnp
BQNPNPgPhBBhCJQhbCgCnrrnbZnlWznzpGssllVW
LFNZRvCPCFZmHmTLcwdwcq
zQRDChCnVhqRllpzQlzqCVVzPvNHTTFFHlNPsPNvTBPBHBFT
wcfdfDmfPHjdsHPs
cwGMwMMbwbmwJZfDSMmfwcpRVWzQqnnnzGVVnhqRVCph
CScCSPcPszFJWSMjGZHMpGMjvG
TTQfQvBTVBfrDVRDVqGMmjZqnpnGmMpnnpZZ
NDdbrQdVQDNNfvTVNdBfrDBJFcsLWcCJzWFCcFLbFcFJsc
WWdzhpHcHHrtzGBJMwmjJGmBtP
CqsgqNrVnlCBCvPjCBmPJm
LDDDLnVsqsgTQsgFcQzbQdprbWHzdb
nFpNPGLGrntlrFLpflfBTcJwSgwQvGwbgSvbvJvT
DHZWmMDZZDHPMHsDJQbbgSJcvbQgjwJM
CRdzzdDDVRHWWNlPfPzllnFLlL
ZJVqGSVCJCSgdSvtjtvcjcjbNl
pmDWFnDFMnDdFLDHffjcbjjtBNctBjBMbMcl
dLnfmdLLmrWsgZqCThgssgsq
CLsvLLQvrlrrpLpw
tmTHnNtgmzgWmpjlpjnwrrlRjw
WHgTdmNJmJTDDrtFWsVGQqBQqcCCbhdvCB
jwCHwmWRTWRWJwfcgVgflvPqPBPH
MpphdbZZpZMNZpsdpZLMgjBlBqDvLPjfLVPBgPfv
ZdrZsjQbnMCGwrRwzmTr
CMDsWppsfhjNNzzzcBrbPgnrrnVBQHBNrV
ZmGdTdvtStFDtTvtBHgPHnPHgnrnHVmH
ZtLZdwdGqtZqLTLtwvfjDWcRscqMCDsCfWjf
QQPPfPjLNLfSShfNRSRrrGHzvHrvlvnHRDJJ
WCMJJVBwbGCbnbCl
gBJmTBBMchsSjgfS
NvCQGNRQQrWRpWhhvQrNLgrJqTVzzLFZsJqJZFcJTqFMJJ
ttdBllbCSdcsJVVVzVMb
CnwfwwdlnPGQgpnPvv
WQmnmBBmWRCgDpndbD
SvjsqGGqTSTdbggS
lZbFJqLjvHZrcZNHcc
gcgQhclQlntnnvBMZlwffTBLwv
JqzNpqbmpJCbbzCfNFFqfWpZvDVTqBMTrTLvrTMTrvrZTT
NbJWCpRWSFWNWJCzmRNRdchcdHcchnfncnhntcdjng
JvDwhPWPzvzPDggWlvCQgPtHgtGnHtNqLqTnHTHHnnVg
jBsfcbpdQsRcsSpRcjZjHNGtnnVncVtqTVVNVNNT
prBbsjQdSbZdSFwMzrhPMWwzwlvC
SStQfWQmJQjjhphQ
sMVTwvLTswLwwqMPnnhglbHgglPh
RDrRVrVCsrssBFWffFCZthfNFN
ZnpgzcqgmhHtfwSDwplwVV
QCrBNLdLrrnSVSrl
LQQLLLjTBjGWnNBBGLFZbGgMcsMsZzbgbZZgsZ
ZnbzbhqPbMrnDGjtQGSRVVMGpf
gFdTlNJFGVGChJCt
HgswWLNdhwTTwWsNNvzPmbvbcZbrsbrmzP
SHpDqcJvBmJgJJHgDphHmvhTddWlLsZTTdFWMMsWtthZ
GPrRCPfRfjjwffjrrnPVPsTLZFwLWMzLdTsTFzzFWT
QPfZVNfbfQnRVjCRrPJBvSJgpDSDBHNgBJpB
mjpcZcHcrqjrNmNpNmptHNHWwvCwgwCgMmQgRQsRMgwMvnCg
DfSfFSVtDSTJVfdGJMwPnMRwnRCPQgMR
DDfzDVdbTbzVDDtrtqlHptqbjbql
lsBgqLqhqqgBBvGLBwQJJHRpJCJDHMwhwz
NCVfntPnVnfrZtfdbTntnnWDDMDwwRRJmJDWRWJmHRzpJD
ZrZbtbSZfdNVfbtCrbPTndsFvFLFSqcGLBFgvgjsgcqj
gwRCCDmlZtPDdtBBPM
VLrjccJVcJTfTtLjscVBHjHPjPQQSPpBHMjBhP
rfcJrzvvctrscvzRqGRCZvmqbmwqZw
nffqBWfRfRnpBfznpWTpTlWNNbcrDcbGbPhrGVwGlllPGN
MsJQsmQLjMMFsmjbPZNcNbrbmGVhZZ
LCJsFSsJFMFpfrTBWSvqpB
vSspfvprpTNTZNTj
LlFQCtnwMqqSmCMPmMSwClCJhBDJZZJZZjBTjQNjJbdjTT
PHtwMLPqCHsfVRGvSR
HjLDQMjtRvTmfTfmtf
BddChcvvhwhqgmqTfnSmdg
NBVVZJhZVhZsJJzhJZNHrRMPbQvHrjPvPDRvRN
GgwtwntLQmNjvRGJGv
wwzWzrzrqWjRlhJrNlrj
wBWSCMPMcdnHsTHPsn
MggMDDJzdbvsjCJvWJnJ
THBGGScfHwHqHGTGTBqfSWCsnWBCbvsbshjRnZWZCn
HwltftNGtmHHGqNlGmfPrVDDgzpVplpzDpVPbP
shMdsCMpQMCZMQsZQVDJnvvPpPLSvLSLLV
RmgzGTGRmClLNCvzVn
rCrRftmftWjbTttjcdFZBcjdsFqZQFjZ
qLwvNLtLvPGqSltLLqvNSpflMfQfMpMnBBggslMpfB
VDcVzzDRRVTDgMPBBgdfQD
VVWrcHbVzrzVjHPNqSmtqJJwjLGN
nBswlBBhntTttbFVnt
JNjTHZvLHDLVVLtCCFFPLz
THvNddgZWQNDNZgjZSfGwfsBrhmfGBhcWs
cCSbPmJqwqJjgJtTzJZT
BpBTFFTQZzQgNgZv
sFRVpsfrfrLfslwTcTccwcnCcTRC
DLjLwDPjVPnWWvVWVjcqzSCJTzSzMTtSrqRqJn
bdNGgmbGHdZdHbSRqrTJzrMtTR
dhGmFffGQsWjshcVpt
TrGzZpzWhCHcPPpF
sqsJqtlqDVDJVJttgNSbsQPfcjQcBQjhFFRSPQHFRc
VgqbtllbdvNtDdLZrHWGmwdrLM
DNrqBvvZZNDDHBFJmMNbLMRctztznRsbds
hlSlPPWfQCChPtWWfjTwLzbdRcbMsbMjnRLsgzsz
PCfWwlQpTTVmmtqmVGrvmt
lLrlLRbgrjRbRjFlRrnRRdgPdWdCwdWqmWPwqScdPc
tDZLNGHsNQZDNBGHTCVSddVCPwVqVtcdWV
BhGLBNvhHDTZDjfRvbfjljbbJf
McRctHfDctZGlZZWgpcW
TTQLQqLSLqTqhdLqPzLqLwrpsGpWVGFZGrlgWZGhFllV
QNdzbQgzSvqPzqNqvnnbtfbRfDntCfMRjn
bHQvFvffjpVvPSTvVm
RDLRRDLBnVbsDsqDSb
MtGGtJJnwnLltblMFCHfQcthjNfjHCQN
FQfFldFCSRRFQSQmLVfRGwGDDGZDhDGwmmhZtthZ
cTpscpCcNzNNvCBgbPPwvZGZggbghb
TNzznnccTjTspHWNzCTpNlRRQnlfdVFVfLLJFrFJQQ
LsMMLCQQQMTJnJMnsJlGlZJNvbNVGNNSDJ
fhcBqmfvmtRftcmfVVNNVbZSSDVGZmgG
BztFRjFjBchWzhvWTHWTMCdwTdTQnH
cpddMRdHTSNRtRztbG
mWvrQnQFhFNtsSNV
vrBnlLvWvlPCCnrrmBrnQQLmTMHwHHccMgggTNjdcDwjNgCw
hlRhqRnQQHcbBHGVVgRLVmrRgrLR
sCFMTMwtzFMzTwCsswWjCsdQJmJLGfrrgLfgPmPVVVmtLP
TMDjCFMTHShBQSDB
fcpssfGWpRDnvDRWvD
LMvQlPLtLQZbQjBqZBFnCRFZRFzR
jlQrvjlbjbLffpdpdrmGwr
hbRTjRRZthvSDvDn
LGrwPfrGfLjqvBsHzBrvstBv
mpCLCqqqLcwcwCLmLmwwdWgWgVVMTbbWbRdQVNjC
QLCqzhzQDqhHsCJjTcVdScccSVgs
NMPBBfwbmwmmGnfpTJJJcTrRjRJJddpr
mflmdfBBlnPBvBNGnwlGzhDqzHZzZtCLZqqltHtL
GGsFmSmFHHGZsqhSTQjlNQNzpptD
JfVJvvJfWMPPnVVJvhlNlltDldzpjpTzQn
LgLcwJffMMLcwPVvPMvsmRqTmbGGssgGssRRqG
ScnbPhwPHPTbwCGJBDtNZZPZDsttNB
FjfQlVVWrQgfQrrWfddnWfQlqJqJMNJBZJqsZtsJjJsvDjJq
lrVngVQpmmmrlnLGLLmTzchcwhHh
QZtDJqWZtWGmhJJjvVBP
crNMSpcdNNFcrdzlrsVGVnvhmnvHbjVjjrhV
NTszNMSpwTNFzcsTlsLRQLtWwZDDfQgZhggQ
mfmdLLLqsvZzjfPgPT
hppLhBNpHGrhHnQQhGMDhPbPbWzvZzbMzZFPbvbvgz
BNGGNLGcQpCcSstSRmct
mnjsJBjBRsmFsSRqqrGfrqqtrfrN
LZDHZZzdcdQzLbcgLwGtNVrlGrMVNfrllHrH
DPcZZzdQpZPzQQDpDdcpbcPgmBBBsnWfTBmnJmCsjjsPSTjJ
jwwHjCPvLVmhmRdJvr
gTBnbFGTTQMgnTbdbhHhrplhJdlV
DTScZTgSSnGTBFzjDwtLtDDHqwwL
RZWhWWRSgBRrdMRdCmtcdtLncHndqF
bssspTjbVDQGTVGTQsJpbvLtVmFLqqHgnLmCcFnmHC
DpfQJJQpDsGzzfDNhPBlPlMBNhghPZlW
VzJrJMBntJpMnBBJMDDGDQLLQwwDpQLGLG
WSFCWNWWWhQRzvNqLTRD
CbSlSlWHghChhWlcclgcWdHgfrmbmznnffMZMztrMbrJBBBJ
gtjBNTvDQNBPlBFlPFZPdP
mCmpfJCVmMzmfsHpCWdGPWCCWZGtSFtF
zJnhJnJzbbMMpnspmmfsJpLNgqtrjQvrTbTgLgtqgqLj
CLJnZZCJLJZJgZZZJMLSTgnRdFWpFdfFWBffpqDBfjFWQdRD
slNzNzzwwPQrfWmDmqBFWlDm
zwHhsVHwQcHJCgnngMZn
wLRLLddJLdZZZjHdRwgJsjqDVsDVSVGSscsVVmDq
tvMWfnhMvnvztzzVSVqqmcSSVsDGmW
TmhhpnnFlfMzMMRZRPTJRHwdPJZB
sLsQSLvcSrbQbFGlWlCD
BhgPBqBhPwmmpPlGhZMVCFFlbZGF
nCBBTPBHdHwmfCvTLStLJSctrJtt
vwNjwvBSSNndtdBJMJsLvZsJhZpPLM
TmCQDHGTVTLhPQhpZprq
CbCmmTzfVGfFGGCNbctwNPtcSnbjww
NWQQdHdTddhGrnJjqCRggvRmhzmm
wLLVHcFFFfwtFfJzqRvmRqzRVgVz
MlwFfflbLFfbwctDplwcwFMMNNWdWNTNSWDQQnsQNWWHsrQH
QlfbQrBjBQvfDBjhlpwpqbMzwWppGWqGwG
VJNcVCJgcntgRcsZWpPDPDqzPTqqnTpL
ZCRNRNmcJZCcNNVRmVdmHfjdrlfhSrvBDSfH
MzzPjGpjpGPPjdtHBfBNBQBrbtlclV
FqCnZcgcnFsWqmVHVHlgfBrbQVVb
STmCLFZWnTsWvdvSSdvPpGcP
lNjczlDNCSRMSmlR
VbhwhgwGQgwpvQpVDpSCbmfnCPfnMnmPmmBn
VhvGTdhwQwVVVsQQshjcDZdHdqzcDdJrjjzH
DhPffCSLCPCwfPPqqwqVjHFjzljppl
TBWBRWTMRBTTBTBdbQqztzQtWqzQFqbV
sgGTBGTGmNvrGfrz
hJgqGzqQmGQMQzgGmJGhJQSvZPfppjjPnZNTTTTpjWJvWp
lbwbRsRdbdmLdrllbbDcrHwwnjvjNfPWWZwWPjPjjZnTZZpf
dDDrdlDBHbDtVFSqmBShMMVF
ddvtMZJdJTtDvgtfZJfvtWZlHpGljLRcBcjplLwGRnnLGlGc
SNbFbrCNhQbrVQCQSCVzbLjBGRGRGwTBVwpnRGcHcw
bbhrTzrbrPrSQFrTTCmFQPCJmJqvJfZZWftZmZJqDvgfJv
vWLsTNNscttvNTLTLHRgcdqBnVSZVZVWVZqdSdJwwd
jDrFMhGPbGGFFPChDGpGBdngZMzVSSwZgJSqgMSV
bjjPQjphCPprhFrCCjFPDCTmcLgLtsTlmsNsQTtscQHt
PdhqLdNccGsrNLpScBnDznjnBnzppQwpHz
ftFtMbtfRMMWTvfRgRWbWMTjjzngzCzjzmDPjjwQDCDQmm
MRWVVfWfbFWWTbFWlvvRWPJGSLSLNJcPVsqJPcLSGr
fdRbPbHmnqvrvHDz
psTdcMgjjNpllVVgjJslMdpMhrttthznDttBWttBvWcnzBvh
gJjgVNFppjgCCVNsTTCsZbFwmmmmQGZSZLfwSZZd
csDFpcpJFbccqpFqpfggJJsljhvlTvQQtjwPTmjPPjRTtNvh
ZBGLddVCSVwNThhCQjPj
ZHLLLrSHGSBzWWzHWpgwcFbJsfcgJbsrgg
QJljRQLGJSNjMjQBLLJllFznzVCFpBnnzgwngpDCnD
mHWrTmWrdZHWvdrdWrdZttsFspFVcpzcwcggzpwzwVwDVp
vfzWvqTWWtPffWHqrWTZvTNSjRjQGGPPRbJbllQbMlJS
DFnFprBLpHcSlJHRBl
dbdMMCdsVWmMPlHSSVPVJfcf
sllvhgsdLThDnhQF
BSFTWCJWFJmBJdbcgDHgfDzHbncC
MjMPNjhlslPPLjPqPqVcDttzLBHcgDggDggDbB
MZhjMsBNNMhPrNjBrMhMPZWRmFmQFRRFFFQmWFQGFQ
NFgqSSrtlNbNffffffmFFZCf
WPvTBPPnBWmdJjCsPmVd
zBwhwwTRWwhvvzTvnhCTnCnSbbqlNStNbLltDRDHRHqtDR
NgggqJTHTJscdjggNVDVRcNHGLQWqpffZQGQGqpQWpWwQZfW
vvBPBhBFrzvnzSSrrSzPMtWZZcWGwmLnWQWpQlwGwpnl
rzbSCtrSttMctvvFMvrvPvgNJDRNHDsHNJsjTjsJJsVb
HbGVfpJbmbpHLBfHbdChRDDwDRhFlMlFVDFr
QzNQqcNgtqcNMjgqtntsgswRDDRQlFDSFQPDFRDwhCFR
ngscZtsqsznnnszqTnnqHfLWLWLWZmJpWMJGLBZb
cTNmqSbTBFhBQZjq
vswHWHWzHMMttvGGwgppttRfZFZjfBnQnfQZBhwQfhRB
lglsvMHHWHsWjHMtsvHvjWvTmJbCmcCcbCTcJlTmSmPSPT
JjSBbBLppbrvZGhhhvGwZNRtNMPCqCPqRgCFRNMgjP
lWlmDsdDnszRRwdCcNcdwd
TsDwwVTWsHTmTWVzQflQJbSGhZrZZQBhvBSrJZSr
JcrncrnrcZcGtJzfrrrzqbTWTlvW
SCRSDRPSLgRDCHdjjgmdDSHqQTzlmTNNVWbqfbvQzQlbfb
gpvpHHPPLdLRCSgjpLPtpZZMtnBMZJZBZBtMZs
lQSvJllvHBPPHPHWSPQQJtDtnhbwDDwwtwfhrrVw
MgLLdsMsgpRpTLLMgFrbFnfhNbbrhtwDwDDr
MqpTpqGRLpMgBzGPSvlhSBhW
NqpNNNPzhwzzshPwRPHWRmRFQWHDQPHD
VcbcnbjbbrrbbcnbZQDHmlRSVlvZWRFm
MttmjTtMrhqwhLhtdN
dMggwDwvMdqgqqtqwHnzVnmGmGtGRrFmsJrN
SlBClclffBPfZlssrGnJnZZzzznF
ljfTPplCpLcpBBPfTBfcCTbvbDMMFhqdvqWpMWHMDMpd
llTNZlhSvqMGlZMGhGgGlttrbVVwNjDbNFJbtjjDtN
fQWCBWQBBpBCsmzPmnmddQccwJtjdVbJDtDcrrjwjbFr
QzpQCRspPPPmzfppmmBBWLhgTZZTMRwSTgLlqvMqGG
HSfnNllsHThcchcJBjJhRL
FQQdzFCrFMbdFbrJBcqprcBrcBDqcB
mtmMFJJmnTfnsHvm
nddbfrBHdvbdBBhhhnWmtLsBGQCCtpmmMGPMQP
cwVVqVNggDgjZDFspptttQrsPgpttp
wDjczVZqSFDZVlSvrfhHlJHThh
CttLqSPLqLHhhCdGGTgdlZfclNlsfglbTg
nzFJpQJWVQjFmnmpjFWzVvBbMBTBNNTfTgZlTgFMNNZb
njDDWpVWQjQnzZzjJjnRRwCqPtLLrSqhSwCPCHqG
BFmNvfFNJRrdpMLLLVldWm
jPjGGwqsndHZqJqM
GtTSbjbtJjGQsQSSbPSGbzNgCRcCTCFhvfFgFFTBhfRc
rwBvGlDrBMSzMvGVSBwMSZgnJmmJqmcTTTmVCFJRcTgF
bbjpzLhHnFTLngLJ
zdHQdQQftWfNfNtsSvrsGBPGlswrlvDw
hdnMhghHZzFnZhDCCVTTTbtVmHmWbT
PQscSNcllJwzwbGmTmWVGm
sjprJpJjJNPpJBlpdFdgMzDDDhhvhd
SMwBWSBMPSfzqzPf
gJDlrFFQlgbFgvjDjTgrQQTGlhNNLdlqLGhNPqwPNPNfGf
jvwHQQjrjrJrppjvJpDFgDgHcsZZnnsVHsmMWCnsBsVVWZ
cblRJczlcBtBRCqNfGgHfpHCVHGp
WZWsLWmSPjMdWFGHTVNgLfvHqGDT
ZdmsFmZPmnZMsWWNsNjdmmmcQlrBQnBtQJtclBrtJwBzJz
wtMNCNwNqwtMMRnVcTlFtlcnFlsl
vrvrjzjZDDDwmwwVFT
jzrHjrrHjjLBPfQPjZBZzvpSMwCNRpRwSdRCNLqSShSR
BDgnhMDCDDpjDhBDJDfMSsLSZzCFTTLzTFLzTFZS
lrqrlmqbvtvWwVRtwlmrrqNvZlZSsLTTFTFZSPzZFzzTBFzz
HRRRbVmmwqhHHHhGJBcD
MvnmMvNjvvvmNnRcvzHgzMGtzhffHwHtwt
ZBBsFPPrTgCpSSBwHrLtwbHbLLtzLf
ZBZZssBdWRJgmgJdNn
TTLChzhDnjQLTDhTQJrzSbbJHsGrGrGFGb
BfvvpflfWVlVsFFvJHcFJFrJrt
ZwMBwwZPWMMpffflqlZMRnRNQLCNhPhDDNssnRQD"#;
