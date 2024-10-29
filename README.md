# Cosmos Cross-Chain Bridge

Bu proje, Cosmos ekosistemi için basit bir cross-chain köprü kontratıdır. Kontrat, zincirler arası token transferini yönetir.

## Özellikler

- Token transferi başlatma ve tamamlama
- Transfer durumu takibi
- Köprü aktivasyon/deaktivasyon
- Transfer listeleme ve sorgulama

## Kurulum

1. Rust ve Cargo'yu yükleyin
2. Repository'yi klonlayın
3. Bağımlılıkları yükleyin:
```cargo build```

## Test

Testleri çalıştırmak için:
```cargo test```

## Kullanım

1. Kontratı deploy edin
2. Transfer başlatmak için:
   - InitiateTransfer mesajını kullanın
3. Transferi tamamlamak için:
   - CompleteTransfer mesajını kullanın

## Lisans

MIT