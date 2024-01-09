package leet.code.java.interview;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.stream.Stream;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

public class MajorityElementTest {

    MajorityElement majorityElement = new MajorityElement();

    @ParameterizedTest
    @MethodSource("numsProvider")
    void testMajorityElement(int[] nums , int expectedOutput) {
        // Given
        // When
        int result = majorityElement.majorityElement(nums);
        // Then
        assertEquals(result, expectedOutput);
    }

    static Stream<Arguments> numsProvider() {
        return Stream.of(Arguments.arguments(new int[]{3,2,3}, 3), 
        Arguments.arguments(new int[]{2,2,1,1,1,2,2}, 2));
    }
}
