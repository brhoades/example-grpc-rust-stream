package main;

import (
  "fmt"
  "log"
  "net"
  "os"
  "time"

  "google.golang.org/grpc"
	"go.rippling.com/devices/rpx"
)

type server struct {
  rpx.UnimplementedRipplingServer
}

func (*server) Agent(srv rpx.Rippling_AgentServer) error {
  fmt.Println("agent connected");

  done := make(chan error)
  go func() {
    for {
      msg, err := srv.Recv()
      if err != nil {
        fmt.Printf("agent recv errored: %e\n", err);
        done <- err
        return;
      }

      fmt.Printf("got msg from agent: %+v\n", msg);
    }
  }()

  go func() {
    for {
      time.Sleep(5 * time.Second)

      msg := rpx.ServerMessage {}
      if err := srv.Send(&msg); err != nil {
        fmt.Printf("agent send errored: %e\n", err);
        done <- err
        return;
      }

      fmt.Printf("sent message to agent\n");
    }
  }()

  err := <-done
  return err
}

func main() {
  if len(os.Args) != 2 {
    log.Fatalf("Exactly one argument is expected, a port to listen on. got %d ags", len(os.Args));
  }

  port := os.Args[1]
  l, err := net.Listen("tcp", fmt.Sprintf(":%s", port))
  if err != nil {
    log.Fatalf("failed to listen on %s: %v", port, err)
  }

  s := grpc.NewServer()
  rpx.RegisterRipplingServer(s, &server{})
  log.Printf("server listening at %v", l.Addr())
  if err := s.Serve(l); err != nil {
    log.Fatalf("failed to serve: %v", err)
  }

  fmt.Printf("Client running on :%d\n", port);
}
