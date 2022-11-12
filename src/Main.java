public class Main {
    public static void main(String[] args) {
        var start = System.currentTimeMillis();
        var sorted = BatchSort.sort(new RandomIterator(1000000));
        var end = System.currentTimeMillis();

        while (sorted.hasNext()) System.out.println(sorted.next());
        System.out.println("time: " + (end - start) + "ms");
    }
}