// const symlink = require("fs").symlink;
//
// symlink('./a.txt', './b.txt', (err,sss)=>{
//     console.log("xsxs",err,sss)
// });

async function test(num) {
    if (num <= 1) {
        return num
    }
    return await test(num - 1) + await test(num - 2)
}

test(100).then(res => console.log(res)).catch(e => console.log(e))