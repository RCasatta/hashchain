import javax.xml.bind.DatatypeConverter;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Random;

public class Main {


    public static void main(String[] args) {
        try {
            byte[] b = new byte[32];
            new Random().nextBytes(b);
            MessageDigest md = MessageDigest.getInstance("SHA-256");
            long start = System.currentTimeMillis();
            for(long l=0;l<1000000000;l++) {
                if(l%10000000==0) {
                    String s = DatatypeConverter.printHexBinary(b).toLowerCase();
                    long elapsedMicros = (System.currentTimeMillis()-start)*1000;
                    System.out.println(l + " " + s + " " + ((double) l / (double) elapsedMicros) + " Mhash/s");
                }
                md.update(b);
                b = md.digest();
                md.reset();
            }
        } catch (NoSuchAlgorithmException e) {
            e.printStackTrace();
        }
    }
}
