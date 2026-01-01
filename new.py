def palin(n):
    x=convert_to_list(n)

    if len(x)<3: return False

    x=list(reversed(x))
    x=convert_to_int(x)
    if n==x: return True
    else: return False

def convert_to_list(n):
    new_list=[]
    while n>0:
        x=n%10
        new_list.append(x)
        n=int(n/10)

    new_list=list(reversed(new_list))
    return new_list

def convert_to_int(n):
    new_int=0
    for x in n:
        new_int=(new_int*10) + x
        
    return new_int

def subsets(given_list):
    length=len(given_list)
    res=[]

    def helper(i,curr_list):
        if i >= length:
            res.append(curr_list[::])
            return
    
        helper(i+1,curr_list)

        curr_list.append(given_list[i])
        helper(i+1,curr_list)
        curr_list.pop()
    
    helper(0,[])
    res.remove([])

    new_list=[]

    for x in res:
        if convert_to_int(x)>9:
            new_list.append(list(reversed(x)))

    return res+new_list

def prime_no(old_list):
    new_list=[]

    for x in old_list:
        x=convert_to_int(x)
        count=1
        for y in range(2,x+1):
            if(x%y==0):
                count=count+1
        if count==2:
            new_list.append(x)

    return new_list

def even_digits(n,k):
    x=convert_to_list(n)
    count=0
    for z in x:
        if z%2==0: count=count+1

    if count>k : return False 
    else: return True

def repeat_digits(n):
    x=convert_to_list(n)
    length=len(x)
    for z in range(length-1):
        if x[z]==x[z+1]: return False

    return True

def original_digits(x,n):
    length=len(n)
    for z in range(length):
        if n[z]!=0:
            if x%n[z]==0: return False

    return True

def verify(prime_no,t,l,k,n):
    final_list=[]
    for x in prime_no:
        if x<=t and x>l:
            if not palin(x) and even_digits(x,k) and repeat_digits(x) and original_digits(x,n):
                final_list.append(x)

    return sorted(final_list)

n=int(input("N: "))
l=int(input("L: "))
t=int(input("T: "))
k=int(input("K: "))
n=convert_to_list(n)
res=subsets(n)

print(res)

res=prime_no(res)

res=verify(res,t,l,k,n)

print("Primes: ",res)
print("Count: ",len(res))



