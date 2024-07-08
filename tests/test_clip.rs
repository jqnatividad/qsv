use crate::workdir::Workdir;

// Assume a user has qsv stats output in their clipboard.
// This test compares the stats output of fruits.csv to the clipboard output.
#[test]
#[ignore = "Requires clipboard to test."]
fn clip_success() {
    let wrk = Workdir::new("stats_clip_equality");
    let mut clip_cmd = wrk.command("clip");
    let clip_output: String = wrk.stdout(&mut clip_cmd);

    #[cfg(not(windows))]
    let expected = "field,type,is_ascii,sum,min,max,range,min_length,max_length,mean,sem,stddev,\
                    variance,cv,nullcount,max_precision,sparsity\nfruit,String,true,,apple,\
                    strawberry,,5,10,,,,,,0,,0\nprice,Float,,7,1.5,3.0,1.5,4,4,2.3333,0.36,0.6236,\
                    0.3889,26.7261,0,1,0";
    #[cfg(windows)]
    let expected = "field,type,is_ascii,sum,min,max,range,min_length,max_length,mean,sem,stddev,\
                    variance,cv,nullcount,max_precision,sparsity\r\nfruit,String,true,,apple,\
                    strawberry,,5,10,,,,,,0,,0\r\nprice,Float,,7,1.5,3.0,1.5,4,4,2.3333,0.36,0.\
                    6236,0.3889,26.7261,0,1,0";

    assert_eq!(clip_output, expected);
}
