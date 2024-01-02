package leet.code.java.interview;

import java.util.HashMap;
import java.util.Map.Entry;

public class MajorityElement {
    public int majorityElement(int[] nums) {
        HashMap<Integer, Integer> map = new HashMap<>();
        int maxKey = 0;
        int maxValue = 0;
        for(int num: nums) {
            Integer value = map.getOrDefault(num, null);
            map.put(num, value == null ? 1 : ++value);
        }
        for(Entry<Integer, Integer> entry : map.entrySet()) {
            if(entry.getValue() > maxValue) {
                maxValue = entry.getValue();
                maxKey = entry.getKey();
            }
        }
        return maxKey;
    }    
}
