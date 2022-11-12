import java.util.*;
import java.util.concurrent.ConcurrentLinkedQueue;

public class BatchSort {
    public static <T extends Comparable<T>> Iterator<T> sort(Iterator<T> input) {
        if (!input.hasNext()) return Collections.emptyIterator();

        var threads = new Stack<Thread>();
        var fragments = new ConcurrentLinkedQueue<Iterator<T>>();

        threads.push(new Thread(() -> {
            var list = new LinkedList<T>();
            list.addLast(input.next());

            while (input.hasNext()) {
                var element = input.next();

                if (element.compareTo(list.getFirst()) < 0) {
                    list.addFirst(element);
                } else if (element.compareTo(list.getLast()) > 0) {
                    list.addLast(element);
                } else {
                    fragments.offer(list.iterator());
                    list = new LinkedList<>();
                    list.addLast(element);
                }
            }
        })).start();

        while (!threads.empty()) {
            if (fragments.size() >= 2) {
                var leftIter = fragments.remove();
                var rightIter = fragments.remove();

                threads.push(new Thread(() -> {
                    var merged = new ArrayList<T>();

                    var left = leftIter.next();
                    var right = rightIter.next();

                    while (true) {
                        if (left.compareTo(right) < 0) {
                            merged.add(left);

                            if (leftIter.hasNext()) left = leftIter.next();
                            else {
                                merged.add(right);
                                while (rightIter.hasNext()) merged.add(rightIter.next());
                                break;
                            }
                        } else {
                            merged.add(right);
                            if (rightIter.hasNext()) right = rightIter.next();
                            else {
                                merged.add(left);
                                while (leftIter.hasNext()) merged.add(leftIter.next());
                                break;
                            }
                        }
                    }

                    fragments.add(merged.iterator());
                })).start();
            }
            else while (!threads.empty() && !threads.peek().isAlive()) threads.pop();
        }

        return fragments.poll();
    }
}
