package leet.code.java.interview;

public class BuySellStock {
    public int maxProfit(int[] prices) {
        int leastSoFar = Integer.MAX_VALUE;
        int profit = 0;
        int profitIfSoldToday = 0;
        
        for(int i = 0; i < prices.length; i++){
            if(prices[i] < leastSoFar){
                leastSoFar = prices[i];
            }
            profitIfSoldToday = prices[i] - leastSoFar;
            if(profit < profitIfSoldToday){
                profit = profitIfSoldToday;
            }
        }
        return profit;
    }
}
