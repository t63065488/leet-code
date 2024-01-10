package leet.code.java.interview;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.stream.Stream;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

public class BuySellStockTest {

    BuySellStock buySellStock = new BuySellStock();

    @ParameterizedTest
    @MethodSource("numsProvider")
    void testMaxProfit(int[] input, int expectedResult) {
        // Given
        // When
        int result = buySellStock.maxProfit(input);
        // Then
        assertEquals(expectedResult, result);

    }

    static Stream<Arguments> numsProvider() {
        return Stream.of(
            Arguments.arguments(new int[]{7,1,5,3,6,4}, 5), 
            Arguments.arguments(new int[]{7,6,4,3,1}, 0)
            );
    }
}
