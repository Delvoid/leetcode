function containsDuplicate(nums: number[]): boolean {
   let set = new Set(nums);
   if(set.size < nums.length) {
       return true;
   }
    return false;
};

function containsDuplicate_2(nums: number[]): boolean {
    let set = new Set();
    set.add(nums[0]);
    for(let i = 1; i < nums.length; i++) {
        if(set.has(nums[i])){
            return true;
        }
        set.add(nums[i]);
    }
    return false;
};