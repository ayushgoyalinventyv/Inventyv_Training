let sum=0;
first_func() 

function first_func()
{
    console.log("First Function");
    let arr1=[1,2,3,4,5];
    let first_element = arr1.shift()

    second_func(first_element,arr1);
}

function second_func(first_element, rest_of_array)
{
    let arr2=[6,7,8,9,10];
    arr2=[first_element, ...arr2, ...rest_of_array];
    console.log(arr2);

    for (let i=0 ; i<arr2.length ; i++){
        sum=sum+arr2[i];
    }
    console.log(sum);
}

const promise = new Promise((resolve, reject) => {
    if(sum>35){
        resolve("Promise Resolved");
    }
    else{
        reject("Promise Rejected");
    }
});

promise
.then((message) => {
    console.log(message);
})
.catch((message) => {
    console.log(message);
});


 