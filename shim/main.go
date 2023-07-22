package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	// "path/filepath"
	"github.com/containerd/containerd"
	"github.com/containerd/containerd/leases"
	"github.com/containerd/containerd/namespaces"
	"github.com/containerd/containerd/oci"
	"github.com/containerd/containerd/snapshots"
	"github.com/opencontainers/runtime-spec/specs-go"
	"github.com/sanity-io/litter"
	flag "github.com/spf13/pflag"
	// "github.com/opencontainers/image-spec/identity"
)

func main() {
	var image string
	flag.StringVarP(&image, "image", "docker.io/library/hello-world:latest", "user image to run")

	var interrupt = make(chan os.Signal, 1)
	signal.Notify(interrupt, os.Interrupt, syscall.SIGTERM)
	// defer signal.Stop(interrupt)

	ctx, cancel := context.WithCancel(context.Background())
	go func() { <-interrupt; cancel() }()

	if err := run(ctx, image); err != nil {
		log.Fatal(err)
	}
}

func run(c context.Context, image string) error {
	ctx := namespaces.WithNamespace(c, "example")

	client, err := containerd.New("/run/containerd/containerd.sock")
	if err != nil {
		return err
	}
	defer client.Close()

	manager := client.LeasesService()
	l, err := manager.Create(ctx, leases.WithRandomID(), leases.WithExpiration(time.Second*15))
	if err != nil {
		return fmt.Errorf("failed to create lease: %s", err)
	}

	ctx = leases.WithLease(ctx, l.ID)

	userImage, err := client.Pull(ctx, image, containerd.WithPullUnpack)
	if err != nil {
		return err
	}

	log.Printf("Successfully pulled %s image\n", userImage.Name())

	desc, err := userImage.Config(ctx)
	if err != nil {
		return err
	}

	log.Printf("Successfully got desc id %s\n", desc.Digest.String())

	cs := client.ContentStore()
	info, err := cs.Info(ctx, desc.Digest)
	if err != nil {
		return err
	}

	log.Printf("Successfully parsed image info\n")

	litter.Dump(info)

	snapshotLabel, ok := info.Labels["containerd.io/gc.ref.snapshot.devmapper"]
	if !ok {
		return fmt.Errorf("failed to find snapshot label")
	}

	snapshotter := client.SnapshotService("overlayfs")

	noGcOpt := snapshots.WithLabels(map[string]string{
		"containerd.io/gc.root": time.Now().UTC().Format(time.RFC3339),
	})

	// TODO: unique name
	mounts, err := snapshotter.Prepare(ctx, "instance", snapshotLabel, noGcOpt)
	if err != nil {
		return fmt.Errorf("failed to prepare snapshot: %v", err)
	}

	container, err := client.NewContainer(
		ctx,
		"vmm", // TODO: unique name
		containerd.WithNewSnapshot("vmm-snapshot", userImage), // TODO: unique name
		containerd.WithNewSpec(
			oci.WithLinuxDevice("/dev/kvm", "rwm"),
			oci.WithLinuxDevice("/dev/net/tun", "rwm"),
			oci.WithLinuxDeviceFollowSymlinks(mounts[0].Source, "rwm"),
			oci.WithEnv([]string{"ROOTFS=" + mounts[0].Source}),
			oci.WithAddedCapabilities([]string{"CAP_NET_ADMIN", "CAP_NET_RAW"}),
			oci.WithHostNamespace(specs.NetworkNamespace),
		),
	)

	return nil

}
