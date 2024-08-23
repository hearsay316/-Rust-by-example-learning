const symlink = require("fs").symlink;

symlink('./a.txt', './b.txt', (err,sss)=>{
    console.log("xsxs",err,sss)
});