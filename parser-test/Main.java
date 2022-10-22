public class Main {
    public static int staticIntReturn() { return 1; }
    public int nonStaticIntReturn() { return 1; }

    public static void staticIntParam(int arg) {}
    public void nonStaticIntParam(int arg) {}
    
    public static void main(String[] args) {
        System.out.println("Hello World.");
    }
}
