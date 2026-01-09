function print_series()
{
    let n=5+1;

    for(let i=1 ; i<n ; i++)
    {
            let temp="";
            for(let j=1 ; j<n ; j++)
            {
                //temp+= Math.min(i+1,j+1,n-i,n-j)+" ";
                if (i<=j && i<=n-i && i<=n-j) {
                    temp+= i;
                } else if (j<=n-i && j<=n-j) {
                    temp+= j;
                } else if (n-i<=n-j) {
                    temp+= n-i;
                } else {
                    temp+= n-j;
                }
            }
            console.log(temp);
    }
}

print_series();

