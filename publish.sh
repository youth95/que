trunk build
pushd dist
git init
git add .
git commit -m "first commit"
git remote add origin https://gitee.com/o9/que_static.git
git push -f -u origin "master"
popd