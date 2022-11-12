import java.util.Iterator;
import java.util.NoSuchElementException;
import java.util.Random;

public class RandomIterator implements Iterator<Byte> {
    private int remaining;
    private final Random random = new Random();

    public RandomIterator(int size) {
        remaining = size;
    }

    @Override
    public boolean hasNext() {
        return remaining != 0;
    }

    @Override
    public Byte next() {
        if (remaining-- == 0) throw new NoSuchElementException();
        return (byte) random.nextInt();
    }
}
