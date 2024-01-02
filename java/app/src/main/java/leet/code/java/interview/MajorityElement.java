package leet.code.java.interview;

import java.util.HashMap;
import java.util.Map.Entry;

public class MajorityElement {
    public int majorityElement(int[] nums) {
        HashMap<Integer, Integer> map = new HashMap<>();
        int n = nums.length / 2;
        for(int num: nums) {
            map.put(num,  map.getOrDefault(num, 0) + 1);
        }
        for(Entry<Integer, Integer> entry : map.entrySet()) {
            if(entry.getValue() > n) {
                return entry.getKey();
            }
        }
        return 0;
    }    
}
