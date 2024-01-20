
fn a() {
    input! {
      n:usize,
      xy:[[usize;2];n]
    }
    let mut x=0;
    let mut y=0;
    for val in xy{
        x+=val[0];
        y+=val[1];
    }
    if x>y{
        println!("Takahashi");
    }else if x<y {
        println!("Aoki");
        
    }else {
        print!("Draw");
    }
}



fn b() {
    input! {
      mut s:Chars,
    }

    // let mut a=0;
    // let mut b=0;
    // let mut c=0;
    // for i in 0..s.len(){
    //     if s[i]=='A' && b==0&&c==0{
    //         a+=1;
    //     }else if s[i]=='B' &&a>0&&c==0 {
    //         b+=1;
            
    //     }else if s[i]=='C' &&a>0 &&b>0 {
    //         c+=1;
            
    //     }else {
    //         println!("No");
    //         return;
    //     }

    // }
    while s[0]=='A'{
        s.remove(0);
        // dbg!(&s);
        if s.len()==0{
            println!("Yes");
            return;
        }
    }
   

    while s[0]=='B'{
        s.remove(0);
        if s.len()==0{
            println!("Yes");
            return;
        }
        
    }
   
    while s[0]=='C'{
        s.remove(0);
        if s.len()==0{
            println!("Yes");
            return;
        }
    }
   if s.len()!=0{
        println!("No");
    }else {
        println!("Yes")
    }
   
}


fn c() {
    input! {
      n:usize,
      a:[isize;n]
    }
    if n==1{
        println!("1");
        return;
    }

    let mut g =vec![vec![];n];
    let mut start=0;
    for i in 0..n{
        if a[i]==-1{
            start=i;
            
        }else{
            g[(a[i]-1) as usize].push(i);
        }
    }
    // dbg!(&g,start);

    // print!("{}", start+1);
    let mut q=VecDeque::new();
    q.push_back(start);

    let mut next_pos=g[start][0];
    q.push_back(next_pos);
    while g[next_pos].len()!=0{
        // dbg!(&next_pos);
        q.push_back(g[next_pos][0]);
        next_pos=g[next_pos][0];
    }
    // dbg!(&q);

    while q.len()>0{
        print!("{} ", q.pop_front().unwrap()+1);
    }
    // println!("{}", next_pos+1);
}



fn d() {
    input! {
      h:usize,
      w:usize,
      k:usize,
      s:[Chars;h]
    }

    let mut num_maru=0;
    let mut max_num_maru=0;

    let mut ok=false;


    let mut q:VecDeque<char> = VecDeque::new();


    // yoko
    for i in 0..h{
        let si =&s[i];
        let mut num_maru=0;
        let mut q = VecDeque::new();

        for j in 0..w{ 
            
            if si[j]!='x'{
                q.push_back(si[j]);
               
                if q.back().unwrap()==&'o'{
                    num_maru+=1;
                }
            }

            // kの範囲から外れたoを削除
            if q.len()>k{
                let out= q.pop_front().unwrap();
                if out=='o'{
                    num_maru-=1;
                }
            }
            if si[j]=='x' {
               q=VecDeque::new();
               num_maru=0;
            }
            // dbg!(&q);
            if q.len()==k{
                ok=true;
                max_num_maru = std::cmp::max(max_num_maru, num_maru);
            }
        }
    }



    // tate
    for i in 0..w{

        let mut num_maru=0;
        let mut q = VecDeque::new();

        for j in 0..h{ 
            
            if s[j][i]!='x'{
                q.push_back(s[j][i]);
               
                if q.back().unwrap()==&'o'{
                    num_maru+=1;
                }
            }

            // kの範囲から外れたoを削除
            if q.len()>k{
                let out= q.pop_front().unwrap();
                if out=='o'{
                    num_maru-=1;
                }
            }
            if s[j][i]=='x' {
               q=VecDeque::new();
               num_maru=0;
            }
            // dbg!(&q);
            if q.len()==k{
                ok=true;
                max_num_maru = std::cmp::max(max_num_maru, num_maru);
            }
        }
    }



    if ok{
        println!("{}", k-max_num_maru);
    }else {
        println!("-1");
    }

    // tate
}