rpx/rippling.go: ../proto/rippling.proto
	protoc --go_out=./rpx/ \
	  	--go_opt=paths=source_relative \
			--go-grpc_out=./rpx/ \
			--go-grpc_opt=paths=source_relative \
			--proto_path=../proto \
			../proto/rippling.proto

