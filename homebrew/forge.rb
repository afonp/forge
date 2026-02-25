class Forge < Formula
  desc "competitive programming exercise scaffolder with a c++ template system"
  homepage "https://github.com/afonp/forge"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/afonp/forge/releases/download/v#{version}/forge-aarch64-apple-darwin.tar.gz"
      # sha256 will be updated by CI
    end
    on_intel do
      url "https://github.com/afonp/forge/releases/download/v#{version}/forge-x86_64-apple-darwin.tar.gz"
      # sha256 will be updated by CI
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/afonp/forge/releases/download/v#{version}/forge-aarch64-unknown-linux-gnu.tar.gz"
      # sha256 will be updated by CI
    end
    on_intel do
      url "https://github.com/afonp/forge/releases/download/v#{version}/forge-x86_64-unknown-linux-gnu.tar.gz"
      # sha256 will be updated by CI
    end
  end

  def install
    bin.install "forge"
  end

  test do
    assert_match "competitive programming exercise scaffolder", shell_output("#{bin}/forge --help")
  end
end
