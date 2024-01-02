package leet.code.java.interview;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

public class MajorityElementTest {

    MajorityElement majorityElement = new MajorityElement();

    @Test
    void testMajorityElementExample1() {
        // Given
        int[] nums = {3,2,3};
        int expectedOutput = 3;
        // When
        int result = majorityElement.majorityElement(nums);
        // Then
        assertEquals(result, expectedOutput);
    }

    @Test
    void testMajorityElementExample2() {
        // Given
        int[] nums = {2,2,1,1,1,2,2};
        int expectedOutput = 2;
        // When
        int result = majorityElement.majorityElement(nums);
        // Then
        assertEquals(result, expectedOutput);
    }
}
