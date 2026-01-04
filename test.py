with open("data.txt", "r") as f:
    curr_list = [line.strip() for line in f if line.strip()]

def seperate(s):
    letter=s[0]
    number=s[1:]
    return letter,number

def result(new_list):
    count=0
    curr=99
    for x in new_list:
        letter,number = seperate(x)
        number=int(number)

        if number!=0:
            fixed = int(number/99)

        if fixed!=0:
            count = count + fixed -1

        add = int(number%99)

        if letter=="L":
            if curr==99 and number==99:
                res=curr-99
            else:
                res=curr-add
        if letter=="R":
            res=curr+add
        
        curr=res
        
            
        if res>99:
            curr=(res%99)-1    
        elif res<0:
            curr=99+res+1

        if curr==0:
            count=count+1
        
            
        print(x,curr,res,count)
    
    return count
        

answer=result(curr_list)
print(answer)



