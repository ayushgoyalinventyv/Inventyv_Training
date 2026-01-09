with open("d1_data.txt", "r") as f:
    curr_list = [line.strip() for line in f if line.strip()]

def seperate(s):
    letter=s[0]
    number=s[1:]
    return letter,number

def result(new_list):
    count=0
    curr=50
    for x in new_list:
        letter,number = seperate(x)
        number=int(number)

        for b in range(number):

            if letter=="L":
                if(curr==0):
                    curr=99
                else:
                    curr=curr-1
                
                if curr==0:
                    count=count+1
                
            if letter=="R":
                if(curr==99):
                    curr=0
                else:
                    curr=curr+1
                
                if curr==0:
                    count=count+1
                
    
    return count
        

answer=result(curr_list)
print(answer)



