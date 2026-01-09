import textwrap

ranges = [
(19391,47353),
(9354357,9434558),
(4646427538,4646497433),
(273,830),
(612658,674925),
(6639011,6699773),
(4426384,4463095),
(527495356,527575097),
(22323258,22422396),
(412175,431622),
(492524,611114),
(77,122),
(992964846,993029776),
(165081,338962),
(925961,994113),
(7967153617,7967231799),
(71518058,71542434),
(64164836,64292066),
(4495586,4655083),
(2,17),
(432139,454960),
(4645,14066),
(6073872,6232058),
(9999984021,10000017929),
(704216,909374),
(48425929,48543963),
(52767,94156),
(26,76),
(1252,3919),
(123,228)
]



def check(y):
    num=str(y)
    length=len(num)

    if(length%1==0):
        sub=textwrap.wrap(num,1)
        if(sub.count(sub[0])==len(sub)):
            return True
    
    if(length%2==0  and length>2):
        sub=textwrap.wrap(num,2)
        if(sub.count(sub[0])==len(sub)):
            return True
        
    if(length%3==0  and length>3):
        sub=textwrap.wrap(num,3)
        if(sub.count(sub[0])==len(sub)):
            return True
    
    if(length%4==0  and length>4):
        sub=textwrap.wrap(num,4)
        if(sub.count(sub[0])==len(sub)):
            return True
    
    if(length%5==0  and length>5):
        sub=textwrap.wrap(num,5)
        if(sub.count(sub[0])==len(sub)):
            return True
    
    return False

def total_sum():
    sum=0
    for a,b in ranges:
        for x in range(a,b+1):
            if (check(x)):
                sum=sum+x
    return sum

print(total_sum())