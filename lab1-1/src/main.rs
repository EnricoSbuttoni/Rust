fn slugify(s: &str) -> String {
    const SUBS_I : &str ="àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    let mut lunghezza=0;
    for _t in s.chars(){
        lunghezza=lunghezza+1;
    }
    if lunghezza==1{
        for chara in s.chars(){
            if !chara.is_alphanumeric(){
                return '-'.to_string();
            }
        }
    }
    let mut flag= false;
    let mut count=0;
    let mut g= String::from("");
    for chara in s.chars(){
        if chara.is_alphabetic(){
            if chara.is_uppercase(){
                g=g+&(chara.to_lowercase().to_string());
                flag=false;
            }
            else{
                let mut t=chara;
                if SUBS_I.contains(chara){
                    t=conv(chara);
                }
                g.push(t);
                flag=false;
            }
        }
        else{
            if chara.is_numeric(){
                g.push(chara);
                flag=false;
            }
            else{
                if !flag{
                    g.push('-');
                    flag=!flag;
                }
            }
        }
    }
    if let Some(supp)=g.chars().last() {
        if supp=='-'{
            g.pop();
        }
    }
    g
    }
    
    fn conv(c: char) -> char {
        let mut res='-';
        const SUBS_I : &str =
        "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
        const SUBS_O: &str =
        "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzz
        z";
        let mut count: usize =0;
        for charac in SUBS_I.chars(){
            count=count+1;
            if c==charac {
                break;
            }
        }
        if count==SUBS_I.len() {
            return '-';
        }
        else{
        for charac1 in SUBS_O.chars(){
            count=count-1;
            if count==0 {
                res=charac1;
                break;
            }
        }
        return res;
        }
    }
    
    
    fn main() {
    
    }
    
    #[cfg(test)]
    mod tests {
    use super::*;
    #[test]
    fn my_first_test() {
    let valore = slugify("Hello WORLèed!!");
    assert_eq!(valore, "hello-worleed");
    }
    
    }
    
    